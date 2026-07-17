//! [`EventBusPublishRequest`] — request to broadcast an event to all subscribers.
// @allow: dto_types_must_serialize — holds a live `Arc<dyn DomainEvent>`, not
// wire-format data; a trait object cannot derive Serialize/Deserialize.

use std::sync::Arc;

use crate::api::DomainEvent;

/// Request to publish `event` to every current [`EventBus`](crate::api::EventBus) subscriber.
pub struct EventBusPublishRequest {
    /// The event to broadcast.
    pub event: Arc<dyn DomainEvent>,
}
