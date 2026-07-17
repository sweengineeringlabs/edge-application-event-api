//! Error type for [`EventBus`](super::super::traits::EventBus) and
//! [`EventSource`](super::super::traits::EventSource) operations.

/// Error produced by event bus and event source operations.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum EventError {
    /// Event serialization or deserialization failed.
    #[error("serialization failed: {0}")]
    SerializationFailed(String),
    /// The event bus or source is unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// The broadcast channel lagged and dropped messages.
    #[error("broadcast lagged: {0} messages dropped")]
    BroadcastLagged(u64),
}
