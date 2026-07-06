//! Internal state container for the circuit breaker.

use crate::api::BreakerState;

/// Internal state container.  Crate-private; consumers observe
/// state via [`GrpcBreakerClient::state`](crate::saf::GrpcBreakerClient::state).
#[derive(Debug, Clone, Copy)]
pub struct BreakerNode {
    pub(crate) state: BreakerState,
    pub(crate) consecutive_failures: u32,
    pub(crate) consecutive_successes: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaker_node_fields_are_readable_within_crate_happy() {
        let node = BreakerNode {
            state: BreakerState::Closed,
            consecutive_failures: 1,
            consecutive_successes: 2,
        };
        assert!(matches!(node.state, BreakerState::Closed));
        assert_eq!(node.consecutive_failures, 1);
        assert_eq!(node.consecutive_successes, 2);
    }

    #[test]
    fn test_breaker_node_open_state_error() {
        let node = BreakerNode {
            state: BreakerState::Open {
                since: std::time::Instant::now(),
            },
            consecutive_failures: 5,
            consecutive_successes: 0,
        };
        assert!(matches!(node.state, BreakerState::Open { .. }));
    }

    #[test]
    fn test_breaker_node_is_copy_edge() {
        let node = BreakerNode {
            state: BreakerState::HalfOpen,
            consecutive_failures: 0,
            consecutive_successes: 0,
        };
        let copied = node;
        assert_eq!(copied.consecutive_failures, node.consecutive_failures);
    }
}
