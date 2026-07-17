//! [`InProcessEventBus`] — in-process broadcast event bus backed by tokio.

use std::sync::Arc;

use crate::api::event::traits::DomainEvent;

/// An in-process broadcast event bus backed by a
/// [`tokio::sync::broadcast`] channel.
///
/// Clone-safe: all clones share the same underlying sender.
pub struct InProcessEventBus {
    pub(crate) sender: tokio::sync::broadcast::Sender<Arc<dyn DomainEvent>>,
}
