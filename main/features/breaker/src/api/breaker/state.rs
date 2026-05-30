//! Concrete state of the circuit breaker.

use std::time::Instant;

/// Concrete state of the breaker.
///
/// Public for observability — consumers can introspect the
/// current state via
/// [`GrpcBreakerClient::state`](crate::saf::GrpcBreakerClient::state).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakerState {
    /// Traffic flows normally.  Failures are counted; at the
    /// configured threshold the breaker trips Open.
    Closed,

    /// All requests short-circuit with `Unavailable`.  After
    /// `cool_down`, the next request promotes to HalfOpen.
    /// `since` records when the breaker entered Open so the
    /// admit-side check can compute elapsed cool-down without
    /// reading a separate field.
    Open {
        /// Instant the breaker last entered Open state.
        since: Instant,
    },

    /// One or more probe requests are in flight.  Successes
    /// count toward `half_open_probe_count`; any failure
    /// returns to Open.
    HalfOpen,
}
