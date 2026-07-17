//! [`AggregateApplyResponse`] — wrapper for a successful aggregate fold.

/// Result of [`Aggregate::apply`](crate::api::Aggregate::apply).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AggregateApplyResponse;
