//! `EventSource` trait — pull-based event source contract.

use std::future::Future;
use std::pin::Pin;

use crate::api::event::errors::EventError;
use crate::api::event::dto::{EventSourceRecvNextRequest, EventSourceRecvNextResponse};

/// A pull-based source of [`DomainEvent`](super::DomainEvent) values.
///
/// Implementors yield one event per `recv_next` call. When the source is
/// exhausted or closed they return [`EventError::Unavailable`].
pub trait EventSource: Send {
    /// Receive the next available event, waiting if necessary.
    fn recv_next(
        &mut self,
        req: EventSourceRecvNextRequest,
    ) -> Pin<Box<dyn Future<Output = Result<EventSourceRecvNextResponse, EventError>> + Send + '_>>;
}
