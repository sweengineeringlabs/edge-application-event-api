//! [`EventStoreLoadFromRequest`] — request to load an aggregate's stream from a sequence.

/// Request to load events for `aggregate_id` starting at `from_sequence` (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventStoreLoadFromRequest<'a> {
    /// The aggregate whose stream to load.
    pub aggregate_id: &'a str,
    /// The first sequence number to include.
    pub from_sequence: u64,
}
