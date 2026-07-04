//! Internal state container for the circuit breaker.

use crate::api::types::breaker_state::BreakerState;

/// Internal state container.  Crate-private; consumers observe
/// state via [`GrpcBreakerClient::state`](crate::saf::GrpcBreakerClient::state).
#[derive(Debug, Clone, Copy)]
pub(crate) struct BreakerNode {
    pub(crate) state: BreakerState,
    pub(crate) consecutive_failures: u32,
    pub(crate) consecutive_successes: u32,
}
