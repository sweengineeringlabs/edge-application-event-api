//! `EventBus` trait — publish/subscribe event bus contract.

use std::future::Future;
use std::pin::Pin;

use crate::api::event::errors::EventError;
use crate::api::event::dto::{
    EventBusPublishRequest, EventBusSubscribeRequest, EventBusSubscribeResponse,
};

/// A publish/subscribe event bus.
///
/// Callers publish [`DomainEvent`](super::DomainEvent) instances and receive an
/// [`EventReceiver`](super::super::types::EventReceiver) handle that produces
/// subsequent events asynchronously.
pub trait EventBus: Send + Sync {
    /// Publish an event to all current subscribers.
    fn publish(
        &self,
        req: EventBusPublishRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), EventError>> + Send + '_>>;

    /// Subscribe, returning a stream-like [`EventReceiver`](super::super::types::EventReceiver).
    fn subscribe(
        &self,
        req: EventBusSubscribeRequest,
    ) -> Result<EventBusSubscribeResponse, EventError>;
}
