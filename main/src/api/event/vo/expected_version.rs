//! [`ExpectedVersion`] — optimistic concurrency guard for event store appends.

/// Optimistic concurrency expectation for an [`EventStore`](super::super::traits::EventStore) append.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExpectedVersion {
    /// No version check — always allow the append.
    Any,
    /// The stream must not exist yet (no events appended).
    NoStream,
    /// The stream's last sequence number must equal this value exactly.
    Exact(u64),
}
