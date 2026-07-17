//! [`EventSourceRecvNextResponse`] — wrapper for the next available event.
// @allow: dto_types_must_serialize — holds a live `Arc<dyn DomainEvent>`, not
// wire-format data; a trait object cannot derive Serialize/Deserialize.

use std::sync::Arc;

use crate::api::DomainEvent;

/// Result of [`EventSource::recv_next`](crate::api::EventSource::recv_next).
pub struct EventSourceRecvNextResponse {
    /// The next available event.
    pub event: Arc<dyn DomainEvent>,
}
