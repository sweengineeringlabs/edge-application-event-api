//! Runnable example: `EventStoreSvc` -- a `Service<EventStore>` in this domain's vocabulary --
//! pairs a concrete `EventStore` with a concrete `Handler` into one object, giving a caller both
//! surfaces (direct append/load, or `Handler`-style dispatch) without tracking two separate
//! collaborators.
//!
//! Neither `edge-application-event-api` nor `edge-application-service-api` ever implements
//! `EventStoreSvc`/`Service`, `EventStore`'s append/load, or `Handler::execute` themselves -- all
//! three, and the actual connection between `EventStore` and `Handler` (here:
//! `EventAppendHandler::execute` appending through the injected store, and reading
//! `ctx.observer` on every call), are this example's own code, exactly as a real consumer's
//! would be. See edge-application issue #150.
//!
//! Run with: `cargo run -p edge-application-event-api --example event_handler`

#![allow(clippy::expect_used)]

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use edge_application_base_api::{DrainRequest, LogEmitRequest};
use edge_application_command_api::DirectCommandBus;
use edge_application_event_api::{
    EventEnvelope, EventStore, EventStoreAppendRequest, EventStoreAppendResponse,
    EventStoreError, EventStoreLoadFromRequest, EventStoreLoadFromResponse, EventStoreLoadRequest,
    EventStoreLoadResponse, ExpectedVersion,
};
use edge_application_handler_api::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer_api::StdObserveFactory;
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

/// Holds its own reference to the same event store `EventStoreSvc` pairs it with. The actual
/// connection -- `execute()` appending into the store, and reading `ctx.observer` -- lives
/// here, never in `edge-application-event-api`.
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

/// Pairs the event store and handler into one object for callers to hold.
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

#[tokio::main]
async fn main() {
    println!("=== EventStoreSvc — one object pairing a concrete EventStore and Handler ===\n");

    let event_store = Arc::new(LocalOrderEventStore {
        streams: RwLock::new(HashMap::new()),
    });
    let handler = Arc::new(EventAppendHandler {
        event_store: Arc::clone(&event_store),
    });
    let service = EventAppendSvc {
        event_store: Arc::clone(&event_store),
        handler,
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] service.handler().execute({{ aggregate_id: \"order-1\", events: [OrderCreated {{ item: \"widget\" }}] }})  -- via Handler");
    let resp = service
        .handler()
        .execute(HandlerExecutionRequest {
            req: EventAppendRequest {
                aggregate_id: "order-1".to_string(),
                events: vec![OrderCreated {
                    order_id: "order-1".to_string(),
                    item: "widget".to_string(),
                }],
                expected: ExpectedVersion::Any,
            },
            ctx: &ctx,
        })
        .await
        .expect("append should succeed");
    println!("[3] service.handler().execute() returned sequence = {}\n", resp.sequence);

    println!("[0] service.port().load({{ aggregate_id: \"order-1\" }})  -- direct EventStore access");
    let loaded = service
        .port()
        .load(EventStoreLoadRequest { aggregate_id: "order-1" })
        .await
        .expect("load should succeed");
    let items: Vec<String> = loaded
        .events
        .iter()
        .map(|e| format!("seq {}: {}", e.sequence, e.event.item))
        .collect();
    println!("[3] service.port() returned {items:?}\n");

    println!("Conclusion: EventAppendSvc gives a caller ONE object reaching both surfaces —");
    println!("service.port() for direct EventStore access, service.handler() for Handler-style");
    println!("dispatch — without event-api itself implementing Service, EventStoreSvc, EventStore,");
    println!("or Handler::execute. The connection between them (execute() appending through the");
    println!("store, reading ctx.observer) is entirely this example's own code, exactly as a real");
    println!("consumer's would be.");
}
