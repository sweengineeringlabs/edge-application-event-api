//! `Aggregate` trait — event-sourced aggregate root contract.

use crate::api::event::errors::EventError;
use crate::api::event::traits::DomainEvent;
use crate::api::event::dto::{
    AggregateApplyRequest, AggregateApplyResponse, AggregateIdentityRequest,
    AggregateIdentityResponse,
};

/// An event-sourced aggregate root.
///
/// State is reconstructed by replaying a sequence of [`DomainEvent`] values
/// through [`Aggregate::apply`]. The initial state is produced by [`Default`].
pub trait Aggregate: Default + Send + Sync + 'static {
    /// The domain event type produced and consumed by this aggregate.
    type Event: DomainEvent + Send + Sync + Clone + 'static;

    /// Fold one event into the aggregate state.
    fn apply(
        &mut self,
        _req: AggregateApplyRequest<'_, Self::Event>,
    ) -> Result<AggregateApplyResponse, EventError> {
        Ok(AggregateApplyResponse)
    }

    /// Return the stable aggregate identity string.
    fn id(&self, _req: AggregateIdentityRequest) -> Result<AggregateIdentityResponse<'_>, EventError> {
        Ok(AggregateIdentityResponse { id: "" })
    }
}
