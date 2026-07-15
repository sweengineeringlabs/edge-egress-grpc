//! Concrete state of the circuit breaker.

use std::time::Instant;

/// Concrete state of the breaker.
///
/// Public for observability — consumers can introspect the
/// current state via
/// [`GrpcBreakerClient::state`](crate::saf::GrpcBreakerClient::state).
///
/// State transitions:
/// - `Closed` → `Open`: on `failure_threshold` consecutive failures
/// - `Open` → `HalfOpen`: after `cool_down_seconds`
/// - `HalfOpen` → `Closed`: on `half_open_probe_count` consecutive successes
/// - `HalfOpen` → `Open`: on any failure
///
/// # Examples
///
/// ```rust
/// use edge_transport_grpc_egress_breaker::BreakerState;
///
/// assert_eq!(BreakerState::Closed, BreakerState::Closed);
/// assert_ne!(BreakerState::Closed, BreakerState::HalfOpen);
///
/// // Pattern match to apply state-specific logic.
/// let state = BreakerState::Closed;
/// match state {
///     BreakerState::Closed    => {} // traffic flows normally
///     BreakerState::Open { since: _ } => {} // reject fast; wait cool-down
///     BreakerState::HalfOpen  => {} // single probe in flight
/// }
/// ```
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
