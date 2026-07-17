//! [`NoopAggregate`] — a no-op aggregate root for use in noop implementations.

/// A no-op aggregate that holds no state and uses all default trait methods.
///
/// Used as a concrete implementation of [`Aggregate`] where a real aggregate
/// is not needed (e.g., test fixtures, structural audit compliance).
#[derive(Default, Debug)]
pub struct NoopAggregate;
