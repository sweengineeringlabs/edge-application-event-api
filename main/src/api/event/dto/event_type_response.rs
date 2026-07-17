//! [`EventTypeResponse`] — wrapper for an event's type name.

/// Result of [`DomainEvent::event_type`](crate::api::DomainEvent::event_type).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventTypeResponse<'a> {
    /// Stable type name for this event, e.g. `"order.created"`.
    pub event_type: &'a str,
}
