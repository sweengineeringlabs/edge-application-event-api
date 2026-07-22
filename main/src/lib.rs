//! # edge-domain-event
//!
//! The event port contracts — event sourcing, CQRS event bus, publish/subscribe.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;

pub use api::AggregateApplyRequest;
pub use api::AggregateApplyResponse;
pub use api::AggregateIdentityRequest;
pub use api::AggregateIdentityResponse;
pub use api::ClosedEventSource;
pub use api::EventAggregateIdRequest;
pub use api::EventAggregateIdResponse;
pub use api::EventBusConfig;
pub use api::EventBusPublishRequest;
pub use api::EventBusSubscribeRequest;
pub use api::EventBusSubscribeResponse;
pub use api::EventEnvelope;
pub use api::EventError;
pub use api::EventOccurredAtRequest;
pub use api::EventOccurredAtResponse;
pub use api::EventPublisherPublishRequest;
pub use api::EventSourceRecvNextRequest;
pub use api::EventSourceRecvNextResponse;
pub use api::EventStoreAppendRequest;
pub use api::EventStoreAppendResponse;
pub use api::EventStoreError;
pub use api::EventStoreLoadFromRequest;
pub use api::EventStoreLoadFromResponse;
pub use api::EventStoreLoadRequest;
pub use api::EventStoreLoadResponse;
pub use api::EventTypeRequest;
pub use api::EventTypeResponse;
pub use api::ExpectedVersion;
pub use api::MemoryEventStore;
pub use api::InProcessEventBus;
pub use api::NoopAggregate;
pub use api::NoopDomainEvent;
pub use api::NoopEventBus;
pub use api::NoopEventPublisher;

// Promoted from saf:: -- these are api/-declared port traits/types this crate
// exposed only via saf:: upstream; api:: is this repo's only surface, so they
// move here unchanged (same symbol, same declaration site in api/).
pub use api::Aggregate;
pub use api::DomainEvent;
pub use api::EventBus;
pub use api::EventPublisher;
pub use api::EventSource;
pub use api::EventStore;
pub use api::EventStoreSvc;
