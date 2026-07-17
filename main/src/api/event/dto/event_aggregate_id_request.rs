//! [`EventAggregateIdRequest`] — zero-sized marker for querying an event's aggregate ID.

/// Request for the ID of the aggregate that produced a [`DomainEvent`](crate::api::DomainEvent).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventAggregateIdRequest;
