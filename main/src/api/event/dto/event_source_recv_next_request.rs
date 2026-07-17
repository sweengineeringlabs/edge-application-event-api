//! [`EventSourceRecvNextRequest`] — zero-sized marker for pulling the next event.

/// Request for the next available event from an [`EventSource`](crate::api::EventSource).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventSourceRecvNextRequest;
