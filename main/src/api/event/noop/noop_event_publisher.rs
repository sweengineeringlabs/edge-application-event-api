//! [`NoopEventPublisher`] — zero-sized event publisher that silently discards events.

/// A zero-sized event publisher that silently discards all published events.
///
/// Useful as a default or test double where event emission has no observable
/// side-effect.
pub struct NoopEventPublisher;
