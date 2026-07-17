//! Noop event infrastructure — zero-sized markers for testing and bring-up.

pub mod noop_aggregate;
pub mod noop_domain_event;
pub mod noop_event_bus;
pub mod noop_event_publisher;

pub use noop_aggregate::NoopAggregate;
pub use noop_domain_event::NoopDomainEvent;
pub use noop_event_bus::NoopEventBus;
pub use noop_event_publisher::NoopEventPublisher;
