//! Integration test: `EventStoreSvc` (declared in
//! `main/src/api/event/traits/event_store_svc.rs`) is satisfied end-to-end by real, concrete
//! pairings — proving both `edge_application_handler_api::Handler` (via `execute()`) and
//! `edge_application_event_api::EventStore` (via direct append/load) are reachable through one
//! paired object, and that the blanket impl over `edge_application_service_api::Service`
//! genuinely grants `EventStoreSvc<S>` for any qualifying pairing.
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use edge_application_base_api::{DrainRequest, LogEmitRequest};
use edge_application_command_api::DirectCommandBus;
use edge_application_event_api::{
    EventEnvelope, EventStore, EventStoreAppendRequest, EventStoreAppendResponse,
    EventStoreError, EventStoreLoadFromRequest, EventStoreLoadFromResponse, EventStoreLoadRequest,
    EventStoreLoadResponse, EventStoreSvc, ExpectedVersion,
};
use edge_application_handler_api::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer_api::{ObserverContext, StdObserveFactory};
use edge_application_service_api::Service;
use edge_security_runtime::SecurityContext;
use parking_lot::RwLock;

#[derive(Debug, Clone, PartialEq)]
struct OrderCreated {
    order_id: String,
    item: String,
}

impl edge_application_event_api::DomainEvent for OrderCreated {}

struct LocalOrderEventStore {
    streams: RwLock<HashMap<String, Vec<EventEnvelope<OrderCreated>>>>,
}

impl EventStore for LocalOrderEventStore {
    type Event = OrderCreated;

    fn append(
        &self,
        req: EventStoreAppendRequest<'_, OrderCreated>,
    ) -> Pin<Box<dyn Future<Output = Result<EventStoreAppendResponse, EventStoreError>> + Send + '_>>
    {
        let aggregate_id = req.aggregate_id.to_string();
        let events = req.events;
        Box::pin(async move {
            let mut streams = self.streams.write();
            let stream = streams.entry(aggregate_id.clone()).or_default();
            let mut sequence = stream.len() as u64;
            for event in events {
                sequence += 1;
                stream.push(EventEnvelope {
                    aggregate_id: aggregate_id.clone(),
                    sequence,
                    occurred_at: std::time::SystemTime::now(),
                    event,
                });
            }
            Ok(EventStoreAppendResponse { sequence })
        })
    }

    fn load(
        &self,
        req: EventStoreLoadRequest<'_>,
    ) -> Pin<Box<dyn Future<Output = Result<EventStoreLoadResponse<OrderCreated>, EventStoreError>> + Send + '_>>
    {
        let aggregate_id = req.aggregate_id.to_string();
        Box::pin(async move {
            let events = self
                .streams
                .read()
                .get(&aggregate_id)
                .cloned()
                .unwrap_or_default();
            Ok(EventStoreLoadResponse { events })
        })
    }

    fn load_from(
        &self,
        req: EventStoreLoadFromRequest<'_>,
    ) -> Pin<Box<dyn Future<Output = Result<EventStoreLoadFromResponse<OrderCreated>, EventStoreError>> + Send + '_>>
    {
        let aggregate_id = req.aggregate_id.to_string();
        let from_sequence = req.from_sequence;
        Box::pin(async move {
            let events = self
                .streams
                .read()
                .get(&aggregate_id)
                .map(|stream| {
                    stream
                        .iter()
                        .filter(|e| e.sequence >= from_sequence)
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();
            Ok(EventStoreLoadFromResponse { events })
        })
    }
}

struct EventAppendRequest {
    aggregate_id: String,
    events: Vec<OrderCreated>,
    expected: ExpectedVersion,
}
impl edge_application_handler_api::Request for EventAppendRequest {}

struct EventAppendResponse {
    sequence: u64,
}
impl edge_application_handler_api::Response for EventAppendResponse {}

/// Genuinely reads `ctx.observer` on every call -- proof the composition gains real
/// application infra, not just that the types happen to line up.
struct EventAppendHandler {
    event_store: Arc<LocalOrderEventStore>,
}

#[async_trait::async_trait]
impl Handler for EventAppendHandler {
    type Request = EventAppendRequest;
    type Response = EventAppendResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, EventAppendRequest>,
    ) -> Result<EventAppendResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "event_append_handler".to_string(),
                message: format!("appending events for {:?}", req.req.aggregate_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        let result = self
            .event_store
            .append(EventStoreAppendRequest {
                aggregate_id: &req.req.aggregate_id,
                events: req.req.events,
                expected: req.req.expected,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(EventAppendResponse {
            sequence: result.sequence,
        })
    }
}

struct EventHistoryRequest {
    aggregate_id: String,
}
impl edge_application_handler_api::Request for EventHistoryRequest {}

struct EventHistoryResponse {
    events: Vec<OrderCreated>,
}
impl edge_application_handler_api::Response for EventHistoryResponse {}

struct EventHistoryHandler {
    event_store: Arc<LocalOrderEventStore>,
}

#[async_trait::async_trait]
impl Handler for EventHistoryHandler {
    type Request = EventHistoryRequest;
    type Response = EventHistoryResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, EventHistoryRequest>,
    ) -> Result<EventHistoryResponse, HandlerError> {
        let loaded = self
            .event_store
            .load(EventStoreLoadRequest {
                aggregate_id: &req.req.aggregate_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(EventHistoryResponse {
            events: loaded.events.into_iter().map(|e| e.event).collect(),
        })
    }
}

struct EventAppendSvc {
    event_store: Arc<LocalOrderEventStore>,
    handler: Arc<EventAppendHandler>,
}

impl Service<LocalOrderEventStore> for EventAppendSvc {
    type H = EventAppendHandler;

    fn port(&self) -> &Arc<LocalOrderEventStore> {
        &self.event_store
    }

    fn handler(&self) -> &Arc<EventAppendHandler> {
        &self.handler
    }
}

struct EventHistorySvc {
    event_store: Arc<LocalOrderEventStore>,
    handler: Arc<EventHistoryHandler>,
}

impl Service<LocalOrderEventStore> for EventHistorySvc {
    type H = EventHistoryHandler;

    fn port(&self) -> &Arc<LocalOrderEventStore> {
        &self.event_store
    }

    fn handler(&self) -> &Arc<EventHistoryHandler> {
        &self.handler
    }
}

fn build_services() -> (EventAppendSvc, EventHistorySvc) {
    let event_store = Arc::new(LocalOrderEventStore {
        streams: RwLock::new(HashMap::new()),
    });
    let append = EventAppendSvc {
        event_store: Arc::clone(&event_store),
        handler: Arc::new(EventAppendHandler {
            event_store: Arc::clone(&event_store),
        }),
    };
    let history = EventHistorySvc {
        event_store: Arc::clone(&event_store),
        handler: Arc::new(EventHistoryHandler {
            event_store: Arc::clone(&event_store),
        }),
    };
    (append, history)
}

fn ctx_parts() -> (SecurityContext, DirectCommandBus, Box<dyn ObserverContext>) {
    (
        SecurityContext::unauthenticated(),
        DirectCommandBus,
        StdObserveFactory::noop_observer_context(),
    )
}

async fn run_append(
    svc: &EventAppendSvc,
    aggregate_id: &str,
    events: Vec<OrderCreated>,
) -> Result<u64, HandlerError> {
    let (security, bus, observer) = ctx_parts();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    svc.handler()
        .execute(HandlerExecutionRequest {
            req: EventAppendRequest {
                aggregate_id: aggregate_id.to_string(),
                events,
                expected: ExpectedVersion::Any,
            },
            ctx: &ctx,
        })
        .await
        .map(|resp| resp.sequence)
}

async fn run_history(
    svc: &EventHistorySvc,
    aggregate_id: &str,
) -> Result<Vec<OrderCreated>, HandlerError> {
    let (security, bus, observer) = ctx_parts();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };
    svc.handler()
        .execute(HandlerExecutionRequest {
            req: EventHistoryRequest {
                aggregate_id: aggregate_id.to_string(),
            },
            ctx: &ctx,
        })
        .await
        .map(|resp| resp.events)
}

/// Generic over `EventStoreSvc<S>` specifically (not `Service<S>`) -- proves the blanket impl in
/// `event_store_svc.rs` actually grants `EventAppendSvc: EventStoreSvc<LocalOrderEventStore>`,
/// not just `Service<LocalOrderEventStore>`.
fn assert_event_store_svc<S: EventStore, T: EventStoreSvc<S>>(svc: &T) -> &Arc<S> {
    svc.port()
}

/// @covers: Service::handler
#[tokio::test]
async fn test_append_handler_execute_first_event_returns_sequence_one_happy() {
    let (append, _history) = build_services();
    let sequence = run_append(
        &append,
        "order-1",
        vec![OrderCreated {
            order_id: "order-1".to_string(),
            item: "widget".to_string(),
        }],
    )
    .await
    .unwrap();
    assert_eq!(sequence, 1);
}

/// @covers: Service::handler
#[tokio::test]
async fn test_append_handler_execute_repeated_appends_increment_sequence_edge() {
    let (append, _history) = build_services();
    run_append(
        &append,
        "order-1",
        vec![OrderCreated {
            order_id: "order-1".to_string(),
            item: "widget".to_string(),
        }],
    )
    .await
    .unwrap();
    let second = run_append(
        &append,
        "order-1",
        vec![OrderCreated {
            order_id: "order-1".to_string(),
            item: "gadget".to_string(),
        }],
    )
    .await
    .unwrap();
    assert_eq!(second, 2);
}

/// @covers: Service::handler
#[tokio::test]
async fn test_append_handler_execute_empty_events_returns_unchanged_sequence_error() {
    let (append, _history) = build_services();
    let sequence = run_append(&append, "order-empty", vec![]).await.unwrap();
    assert_eq!(sequence, 0);
}

/// @covers: Service::handler
#[tokio::test]
async fn test_history_handler_execute_existing_aggregate_returns_events_in_order_happy() {
    let (append, history) = build_services();
    run_append(
        &append,
        "order-1",
        vec![
            OrderCreated {
                order_id: "order-1".to_string(),
                item: "widget".to_string(),
            },
            OrderCreated {
                order_id: "order-1".to_string(),
                item: "gadget".to_string(),
            },
        ],
    )
    .await
    .unwrap();

    let events = run_history(&history, "order-1").await.unwrap();
    assert_eq!(
        events,
        vec![
            OrderCreated {
                order_id: "order-1".to_string(),
                item: "widget".to_string(),
            },
            OrderCreated {
                order_id: "order-1".to_string(),
                item: "gadget".to_string(),
            },
        ]
    );
}

/// @covers: Service::handler
#[tokio::test]
async fn test_history_handler_execute_missing_aggregate_returns_empty_error() {
    let (_append, history) = build_services();
    let events = run_history(&history, "does-not-exist").await.unwrap();
    assert!(events.is_empty());
}

/// @covers: Service::port
#[tokio::test]
async fn test_append_and_history_svc_share_the_same_underlying_event_store_edge() {
    let (append, history) = build_services();
    // appended only through EventAppendSvc's handler()...
    run_append(
        &append,
        "order-2",
        vec![OrderCreated {
            order_id: "order-2".to_string(),
            item: "gizmo".to_string(),
        }],
    )
    .await
    .unwrap();
    // ...and visible through EventHistorySvc's handler(), proving both Service pairings share
    // the SAME underlying EventStore instance, not two independent copies.
    let events = run_history(&history, "order-2").await.unwrap();
    assert_eq!(
        events,
        vec![OrderCreated {
            order_id: "order-2".to_string(),
            item: "gizmo".to_string(),
        }]
    );
    // also reachable via direct port() access, same store instance again.
    assert_eq!(append.port().load(EventStoreLoadRequest { aggregate_id: "order-2" }).await.unwrap().events.len(), 1);
    let _ = history; // keep history alive through the direct-port assertion above
}

/// @covers: EventStoreSvc
#[tokio::test]
async fn test_event_store_svc_blanket_impl_grants_bound_from_service_edge() {
    let (append, _history) = build_services();
    run_append(
        &append,
        "order-3",
        vec![OrderCreated {
            order_id: "order-3".to_string(),
            item: "sprocket".to_string(),
        }],
    )
    .await
    .unwrap();

    // proves EventAppendSvc satisfies EventStoreSvc<LocalOrderEventStore> specifically, via the
    // blanket impl -- not just Service<LocalOrderEventStore>.
    let store = assert_event_store_svc(&append);
    let loaded = store
        .load(EventStoreLoadRequest { aggregate_id: "order-3" })
        .await
        .unwrap();
    assert_eq!(loaded.events.len(), 1);
}
