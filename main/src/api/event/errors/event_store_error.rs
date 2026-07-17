//! Error type for [`EventStore`](super::super::traits::EventStore) operations.

/// Error produced by event store operations.
#[derive(Debug, thiserror::Error)]
pub enum EventStoreError {
    /// An optimistic concurrency conflict occurred.
    #[error("version conflict on '{aggregate_id}': expected version {expected}, found {actual}")]
    Conflict {
        /// The aggregate stream where the conflict occurred.
        aggregate_id: String,
        /// The version the caller expected.
        expected: u64,
        /// The version actually found in the store.
        actual: u64,
    },
    /// The event store is temporarily unavailable.
    #[error("event store unavailable: {0}")]
    Unavailable(String),
    /// An unexpected internal error occurred.
    #[error("internal error: {0}")]
    Internal(String),
}
