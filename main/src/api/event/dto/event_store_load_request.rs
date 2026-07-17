//! [`EventStoreLoadRequest`] — request to load an aggregate's full event stream.

/// Request to load all events for `aggregate_id` in sequence order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventStoreLoadRequest<'a> {
    /// The aggregate whose stream to load.
    pub aggregate_id: &'a str,
}
