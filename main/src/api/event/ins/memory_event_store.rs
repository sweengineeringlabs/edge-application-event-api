//! [`MemoryEventStore<E>`] — in-memory event store for testing and development.

use std::collections::HashMap;

use parking_lot::RwLock;

use crate::api::event::traits::DomainEvent;
use crate::api::event::vo::EventEnvelope;

/// An in-memory event store keyed by aggregate ID.
///
/// Uses a [`parking_lot::RwLock`] so reads are fully concurrent.
/// Intended for unit tests and local development; not for production use.
pub struct MemoryEventStore<E: DomainEvent + Clone + Send + Sync + 'static> {
    pub(crate) streams: RwLock<HashMap<String, Vec<EventEnvelope<E>>>>,
}
