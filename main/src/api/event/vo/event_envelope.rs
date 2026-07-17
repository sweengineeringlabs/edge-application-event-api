//! [`EventEnvelope<E>`] — event wrapped with stream metadata.

use std::time::SystemTime;

/// An event together with its position in an aggregate event stream.
///
/// Stored by [`EventStore`](super::super::traits::EventStore) and returned
/// by load operations.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EventEnvelope<E> {
    /// The ID of the aggregate that owns this stream.
    pub aggregate_id: String,
    /// Monotonically increasing position in the stream (1-based after first append).
    pub sequence: u64,
    /// Wall-clock time recorded when the event was appended.
    pub occurred_at: SystemTime,
    /// The domain event payload.
    pub event: E,
}
