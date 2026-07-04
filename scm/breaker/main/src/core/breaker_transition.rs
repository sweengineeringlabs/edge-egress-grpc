//! Pure state-transition logic.
//!
//! No I/O, no clock except `Instant::now()` at the moment of
//! state change.  The decorator in `core::breaker_egress`
//! calls these methods, then stores the returned node —
//! keeping the critical section tight.

use std::time::Instant;

use tracing::{debug, info, warn};

use crate::api::{
    Admission, AdmitRequest, AdmitResponse, BreakerDomainError, BreakerState, BreakerTransition,
    Outcome, RecordOutcomeRequest, RecordOutcomeResponse,
};

/// Default [`BreakerTransition`] implementation — all transition logic
/// lives here.
pub(crate) struct DefaultBreakerTransition;

impl BreakerTransition for DefaultBreakerTransition {
    /// Decide whether to admit a new request.  May promote
    /// Open → HalfOpen if the cool-down has elapsed.
    fn admit(&self, req: AdmitRequest) -> Result<AdmitResponse, BreakerDomainError> {
        let mut node = req.node;
        let admission = match node.state {
            BreakerState::Closed => Admission::Proceed,
            BreakerState::HalfOpen => Admission::Proceed,
            BreakerState::Open { since } => {
                if since.elapsed() >= req.config.cool_down() {
                    debug!(
                        cool_down_seconds = req.config.cool_down_seconds,
                        "grpc-breaker: cool-down elapsed, promoting to HalfOpen",
                    );
                    node.state = BreakerState::HalfOpen;
                    node.consecutive_successes = 0;
                    Admission::Proceed
                } else {
                    Admission::RejectOpen
                }
            }
        };
        Ok(AdmitResponse { admission, node })
    }

    /// Record the outcome of a dispatched request and update state.
    ///
    /// Called only when [`admit`](Self::admit) returned [`Admission::Proceed`] —
    /// i.e. we actually called the inner client.
    fn record(
        &self,
        req: RecordOutcomeRequest,
    ) -> Result<RecordOutcomeResponse, BreakerDomainError> {
        let mut node = req.node;
        match (node.state, req.outcome) {
            (BreakerState::Closed, Outcome::Success) => {
                node.consecutive_failures = 0;
            }
            (BreakerState::Closed, Outcome::Failure) => {
                node.consecutive_failures = node.consecutive_failures.saturating_add(1);
                if node.consecutive_failures >= req.config.failure_threshold {
                    warn!(
                        failures = node.consecutive_failures,
                        threshold = req.config.failure_threshold,
                        "grpc-breaker: failure threshold reached, opening",
                    );
                    node.state = BreakerState::Open {
                        since: Instant::now(),
                    };
                }
            }
            (BreakerState::HalfOpen, Outcome::Success) => {
                node.consecutive_successes = node.consecutive_successes.saturating_add(1);
                if node.consecutive_successes >= req.config.half_open_probe_count {
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
        Ok(RecordOutcomeResponse { node })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{BreakerNode, GrpcBreakerConfig};

    fn closed_node() -> BreakerNode {
        BreakerNode::new()
    }

    fn cfg() -> GrpcBreakerConfig {
        GrpcBreakerConfig {
            failure_threshold: 2,
            cool_down_seconds: 60,
            half_open_probe_count: 1,
        }
    }

    #[test]
    fn test_admit_closed_node_returns_proceed() {
        let resp = DefaultBreakerTransition
            .admit(AdmitRequest {
                node: closed_node(),
                config: cfg(),
            })
            .expect("infallible");
        assert_eq!(resp.admission, Admission::Proceed);
    }

    #[test]
    fn test_record_failure_at_threshold_opens_breaker() {
        let mut node = closed_node();
        for _ in 0..2 {
            let resp = DefaultBreakerTransition
                .record(RecordOutcomeRequest {
                    node,
                    config: cfg(),
                    outcome: Outcome::Failure,
                })
                .expect("infallible");
            node = resp.node;
        }
        assert!(matches!(node.state, BreakerState::Open { .. }));
    }

    #[test]
    fn test_admit_open_node_returns_reject() {
        let node = BreakerNode {
            state: BreakerState::Open {
                since: Instant::now(),
            },
            consecutive_failures: 2,
            consecutive_successes: 0,
        };
        let resp = DefaultBreakerTransition
            .admit(AdmitRequest {
                node,
                config: cfg(),
            })
            .expect("infallible");
        assert_eq!(resp.admission, Admission::RejectOpen);
    }
}
