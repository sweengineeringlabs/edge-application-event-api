//! [`EventStoreAppendResponse`] — wrapper for a successful event store append.

/// Result of [`EventStore::append`](crate::api::EventStore::append).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventStoreAppendResponse {
    /// The sequence number of the last appended event.
    pub sequence: u64,
}
