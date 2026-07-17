//! `EventStore` trait — append-only event stream persistence contract.

use std::future::Future;
use std::pin::Pin;

use crate::api::event::errors::EventStoreError;
use crate::api::event::traits::DomainEvent;
use crate::api::event::dto::{
    EventStoreAppendRequest, EventStoreAppendResponse, EventStoreLoadFromRequest,
    EventStoreLoadFromResponse, EventStoreLoadRequest, EventStoreLoadResponse,
};

/// Append-only storage for domain event streams keyed by aggregate ID.
///
/// Optimistic concurrency is enforced via [`ExpectedVersion`](super::super::vo::ExpectedVersion):
/// callers declare what version they read before appending so conflicting
/// writes are detected and rejected.
pub trait EventStore: Send + Sync {
    /// The domain event type stored in this store.
    type Event: DomainEvent + Send + 'static;

    /// Append events to the stream for the given aggregate ID.
    ///
    /// The expected version is checked before writing; a mismatch yields
    /// [`EventStoreError::Conflict`].
    fn append(
        &self,
        req: EventStoreAppendRequest<'_, Self::Event>,
    ) -> Pin<Box<dyn Future<Output = Result<EventStoreAppendResponse, EventStoreError>> + Send + '_>>;

    /// Load all events for an aggregate in sequence order.
    fn load(
        &self,
        req: EventStoreLoadRequest<'_>,
    ) -> Pin<Box<dyn Future<Output = Result<EventStoreLoadResponse<Self::Event>, EventStoreError>> + Send + '_>>;

    /// Load events for an aggregate starting at a given sequence number (inclusive).
    fn load_from(
        &self,
        req: EventStoreLoadFromRequest<'_>,
    ) -> Pin<Box<dyn Future<Output = Result<EventStoreLoadFromResponse<Self::Event>, EventStoreError>> + Send + '_>>;
}
