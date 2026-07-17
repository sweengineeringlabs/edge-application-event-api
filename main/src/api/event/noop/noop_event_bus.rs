//! [`NoopEventBus`] — zero-sized event bus that silently discards events.

/// A zero-sized event bus that silently discards all published events.
///
/// [`subscribe`](crate::api::event::traits::EventBus::subscribe) returns a
/// closed receiver that immediately yields
/// [`EventError::Unavailable`](crate::api::event::errors::EventError::Unavailable).
pub struct NoopEventBus;
