//! [`EventStoreLoadFromResponse`] — wrapper for a loaded partial event stream.

use crate::api::EventEnvelope;

/// Result of [`EventStore::load_from`](crate::api::EventStore::load_from).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventStoreLoadFromResponse<E> {
    /// Events for the requested aggregate at or after the requested sequence.
    pub events: Vec<EventEnvelope<E>>,
}
