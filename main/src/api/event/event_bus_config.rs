//! [`EventBusConfig`] — configuration for the in-process event bus.

/// Configuration for the [`InProcessEventBus`](super::InProcessEventBus).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventBusConfig {
    /// Broadcast channel capacity (number of messages buffered before lagging).
    pub capacity: usize,
}
