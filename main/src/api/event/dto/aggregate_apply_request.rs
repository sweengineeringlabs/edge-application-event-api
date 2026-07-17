//! [`AggregateApplyRequest`] — request to fold one event into an aggregate.
// @allow: dto_types_must_serialize — holds a borrowed `&'a E` reference to the
// event, not owned wire-format data; a derived Deserialize cannot produce a
// borrowed reference with an unbounded lifetime.

/// Request to apply `event` to an [`Aggregate`](crate::api::Aggregate)'s state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregateApplyRequest<'a, E> {
    /// The event to fold into the aggregate.
    pub event: &'a E,
}
