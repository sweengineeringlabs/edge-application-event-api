//! [`EventBusSubscribeResponse`] — wrapper for a new event bus subscription.
// @allow: dto_types_must_serialize — holds a live `Box<dyn EventSource>`
// handle, not wire-format data; a trait object cannot derive
// Serialize/Deserialize.

use crate::api::EventSource;

/// Result of [`EventBus::subscribe`](crate::api::EventBus::subscribe).
pub struct EventBusSubscribeResponse {
    /// Owned, type-erased handle over the new subscription. Call
    /// [`recv`](EventSource::recv) to pull the next event.
    pub receiver: Box<dyn EventSource>,
}
