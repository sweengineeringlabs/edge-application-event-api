//! [`AggregateIdentityRequest`] — zero-sized marker for querying an aggregate's identity.

/// Request for an [`Aggregate`](crate::api::Aggregate)'s stable identity string.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AggregateIdentityRequest;
