//! [`AggregateIdentityResponse`] — wrapper for an aggregate's identity string.

/// Result of [`Aggregate::id`](crate::api::Aggregate::id).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AggregateIdentityResponse<'a> {
    /// The stable aggregate identity string.
    pub id: &'a str,
}
