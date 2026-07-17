//! [`EventStoreAppendRequest`] — request to append events to an aggregate's stream.

use crate::api::ExpectedVersion;

/// Request to append `events` to the stream for `aggregate_id`.
///
/// `expected` is checked before writing; a mismatch yields
/// [`EventStoreError::Conflict`](crate::api::EventStoreError::Conflict).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventStoreAppendRequest<'a, E> {
    /// The aggregate whose stream to append to.
    pub aggregate_id: &'a str,
    /// The events to append, in order.
    pub events: Vec<E>,
    /// The optimistic-concurrency version the caller expects.
    pub expected: ExpectedVersion,
}
