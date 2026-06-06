//! Internal state container for the circuit breaker.

use crate::api::vo::breaker_state::BreakerState;

/// Internal state container.  Crate-private; consumers observe
/// state via [`GrpcBreakerClient::state`](crate::saf::GrpcBreakerClient::state).
#[derive(Debug)]
pub(crate) struct BreakerNode {
    pub(crate) state: BreakerState,
    pub(crate) consecutive_failures: u32,
    pub(crate) consecutive_successes: u32,
}

impl BreakerNode {
    pub(crate) fn new() -> Self {
        Self {
            state: BreakerState::Closed,
            consecutive_failures: 0,
            consecutive_successes: 0,
        }
    }
}
