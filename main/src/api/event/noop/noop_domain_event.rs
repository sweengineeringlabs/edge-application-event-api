//! [`NoopDomainEvent`] — a no-op domain event for use in noop implementations.

/// A no-op domain event that carries no state and uses all default trait methods.
///
/// Used as the `Event` associated type for [`NoopAggregate`] and as a concrete
/// type in tests where a real event type is not needed.
#[derive(Clone, Debug, Default)]
pub struct NoopDomainEvent;
