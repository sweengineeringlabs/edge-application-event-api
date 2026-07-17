//! [`EventAggregateIdResponse`] — wrapper for an event's aggregate ID.

/// Result of [`DomainEvent::aggregate_id`](crate::api::DomainEvent::aggregate_id).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventAggregateIdResponse<'a> {
    /// ID of the aggregate that produced this event.
    pub aggregate_id: &'a str,
}
