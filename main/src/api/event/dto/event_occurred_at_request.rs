//! [`EventOccurredAtRequest`] — zero-sized marker for querying when an event occurred.

/// Request for the wall-clock time a [`DomainEvent`](crate::api::DomainEvent) occurred.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventOccurredAtRequest;
