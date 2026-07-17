//! [`EventPublisherPublishRequest`] — request to publish a single event.
// @allow: dto_types_must_serialize — holds a live `&'a dyn DomainEvent`
// reference, not wire-format data; a trait object cannot derive
// Serialize/Deserialize.

use crate::api::DomainEvent;

/// Request to publish `event` via an [`EventPublisher`](crate::api::EventPublisher).
pub struct EventPublisherPublishRequest<'a> {
    /// The event to publish.
    pub event: &'a dyn DomainEvent,
}
