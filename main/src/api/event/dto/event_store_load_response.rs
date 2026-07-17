//! [`EventStoreLoadResponse`] — wrapper for a loaded event stream.

use crate::api::EventEnvelope;

/// Result of [`EventStore::load`](crate::api::EventStore::load).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventStoreLoadResponse<E> {
    /// All events for the requested aggregate, in sequence order.
    pub events: Vec<EventEnvelope<E>>,
}
