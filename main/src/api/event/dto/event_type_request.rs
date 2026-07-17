//! [`EventTypeRequest`] — zero-sized marker for querying an event's type name.

/// Request for a [`DomainEvent`](crate::api::DomainEvent)'s stable type name.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventTypeRequest;
