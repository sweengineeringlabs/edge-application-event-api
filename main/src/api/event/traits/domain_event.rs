//! `DomainEvent` trait — a fact that occurred in the domain.

use std::time::SystemTime;

use crate::api::event::errors::EventError;
use crate::api::event::dto::{
    EventAggregateIdRequest, EventAggregateIdResponse, EventOccurredAtRequest,
    EventOccurredAtResponse, EventTypeRequest, EventTypeResponse,
};

/// A fact that occurred in the domain.
///
/// Domain events are immutable records of something that happened.
/// They carry enough information to reconstruct state when replayed in order.
pub trait DomainEvent: Send + Sync {
    /// Stable type name for this event, e.g. `"order.created"`.
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse { event_type: "event" })
    }

    /// ID of the aggregate that produced this event.
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "" })
    }

    /// Wall-clock time at which the event occurred.
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: SystemTime::now(),
        })
    }
}
