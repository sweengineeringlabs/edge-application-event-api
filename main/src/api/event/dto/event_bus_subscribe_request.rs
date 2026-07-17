//! [`EventBusSubscribeRequest`] — zero-sized marker for subscribing to an event bus.

/// Request to subscribe to an [`EventBus`](crate::api::EventBus).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventBusSubscribeRequest;
