//! Pure state-transition logic.
//!
//! No I/O, no clock except `Instant::now()` at the moment of
//! state change.  The decorator in `core::breaker_client`
//! takes the lock, calls these helpers, then drops the lock —
//! keeping the critical section tight.

use std::time::Instant;

use tracing::{debug, info, warn};

use crate::api::breaker::admission::Admission;
use crate::api::breaker::config::GrpcBreakerConfig;
use crate::api::breaker::node::BreakerNode;
use crate::api::breaker::outcome::Outcome;
use crate::api::breaker::state::BreakerState;

/// Decide whether to admit a new request.  May promote
/// Open → HalfOpen if the cool-down has elapsed.
pub(crate) fn admit(node: &mut BreakerNode, config: &GrpcBreakerConfig) -> Admission {
    match node.state {
        BreakerState::Closed => Admission::Proceed,
        BreakerState::HalfOpen => Admission::Proceed,
        BreakerState::Open { since } => {
            if since.elapsed() >= config.cool_down() {
                debug!(
                    cool_down_seconds = config.cool_down_seconds,
                    "grpc-breaker: cool-down elapsed, promoting to HalfOpen",
                );
                node.state = BreakerState::HalfOpen;
                node.consecutive_successes = 0;
                Admission::Proceed
            } else {
                Admission::RejectOpen
            }
        }
    }
}

/// Record the outcome of a dispatched request and update state.
///
/// Called only when [`admit`] returned [`Admission::Proceed`] —
/// i.e. we actually called the inner client.
pub(crate) fn record(node: &mut BreakerNode, config: &GrpcBreakerConfig, outcome: Outcome) {
    match (node.state, outcome) {
        (BreakerState::Closed, Outcome::Success) => {
            node.consecutive_failures = 0;
        }
        (BreakerState::Closed, Outcome::Failure) => {
            node.consecutive_failures = node.consecutive_failures.saturating_add(1);
            if node.consecutive_failures >= config.failure_threshold {
                warn!(
                    failures = node.consecutive_failures,
                    threshold = config.failure_threshold,
                    "grpc-breaker: failure threshold reached, opening",
                );
                node.state = BreakerState::Open {
                    since: Instant::now(),
                };
            }
        }
        (BreakerState::HalfOpen, Outcome::Success) => {
            node.consecutive_successes = node.consecutive_successes.saturating_add(1);
            if node.consecutive_successes >= config.half_open_probe_count {
                info!(
                    probe_successes = node.consecutive_successes,
                    "grpc-breaker: probe successful, closing",
                );
                node.state = BreakerState::Closed;
                node.consecutive_failures = 0;
                node.consecutive_successes = 0;
            }
        }
        (BreakerState::HalfOpen, Outcome::Failure) => {
            warn!("grpc-breaker: probe failed, returning to Open");
            node.state = BreakerState::Open {
                since: Instant::now(),
            };
            node.consecutive_successes = 0;
        }
        (BreakerState::Open { .. }, _) => {
            // record() called while Open — caller should not
            // dispatch in this state.  Ignore defensively.
        }
    }
}
