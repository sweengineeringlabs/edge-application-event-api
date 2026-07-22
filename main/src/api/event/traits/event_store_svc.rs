//! `EventStoreSvc` — a `Service<S>` where `S: EventStore`, in this domain's vocabulary.

use edge_application_service_api::Service;

use crate::api::event::traits::EventStore;

/// Adds no behavior of its own — every `Service<S>` where `S: EventStore` automatically
/// satisfies `EventStoreSvc` via the blanket impl below. Exists purely so `event-api` and its
/// consumers can bound on a domain-recognizable name instead of the generic `Service` from
/// `edge-application-service-api`. See edge-application issue #150.
pub trait EventStoreSvc<S: EventStore>: Service<S> {}

impl<S: EventStore, T: Service<S>> EventStoreSvc<S> for T {}
