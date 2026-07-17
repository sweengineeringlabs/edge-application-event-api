//! [`ClosedEventSource`] — a zero-sized event source that is permanently closed.

/// A zero-sized event source that is permanently closed.
///
/// Every call to [`EventSource::recv_next`] returns
/// [`EventError::Unavailable`] immediately.
pub struct ClosedEventSource;
