//! `EventPublisher` trait — fire-and-forget event publishing contract.

use std::future::Future;
use std::pin::Pin;

use crate::api::event::errors::EventError;
use crate::api::event::dto::EventPublisherPublishRequest;

/// Publishes a single [`DomainEvent`](super::DomainEvent) without returning a subscription handle.
///
/// This is the simpler outbound port for event emission when the caller does
/// not need to receive events back.
pub trait EventPublisher: Send + Sync {
    /// Publish one event.
    fn publish(
        &self,
        req: EventPublisherPublishRequest<'_>,
    ) -> Pin<Box<dyn Future<Output = Result<(), EventError>> + Send + '_>>;
}
