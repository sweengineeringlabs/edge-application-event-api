//! `Event` theme — event sourcing and CQRS event bus contracts.

pub mod dto;
pub mod errors;
pub mod ins;
pub mod noop;
pub mod traits;
pub mod vo;

mod closed_event_source;
mod event_bus_config;

pub use closed_event_source::ClosedEventSource;
pub use event_bus_config::EventBusConfig;
pub use dto::{
    AggregateApplyRequest, AggregateApplyResponse, AggregateIdentityRequest,
    AggregateIdentityResponse, EventAggregateIdRequest, EventAggregateIdResponse,
    EventBusPublishRequest, EventBusSubscribeRequest, EventBusSubscribeResponse,
    EventOccurredAtRequest, EventOccurredAtResponse, EventPublisherPublishRequest,
    EventSourceRecvNextRequest, EventSourceRecvNextResponse, EventStoreAppendRequest,
    EventStoreAppendResponse, EventStoreLoadFromRequest, EventStoreLoadFromResponse,
    EventStoreLoadRequest, EventStoreLoadResponse, EventTypeRequest, EventTypeResponse,
};
pub use errors::{EventError, EventStoreError};
pub use ins::{InProcessEventBus, MemoryEventStore};
pub use noop::{NoopAggregate, NoopDomainEvent, NoopEventBus, NoopEventPublisher};
pub use traits::{Aggregate, DomainEvent, EventBus, EventPublisher, EventSource, EventStore};
pub use vo::{EventEnvelope, ExpectedVersion};
