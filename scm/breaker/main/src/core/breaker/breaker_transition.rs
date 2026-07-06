//! Pure state-transition logic.
//!
//! No I/O, no clock except `Instant::now()` at the moment of
//! state change.  The decorator in `core::breaker::breaker_egress`
//! calls these methods, then stores the returned state —
//! keeping the critical section tight.

use std::time::Instant;

use tracing::{debug, info, warn};

use crate::api::{
    Admission, AdmitRequest, AdmitResponse, BreakerDomainError, BreakerState, BreakerTransition,
    Outcome, RecordOutcomeRequest, RecordOutcomeResponse, BREAKER_TRANSITION_LOG_TARGET,
};

/// Default [`BreakerTransition`] implementation — all transition logic
/// lives here.
pub(crate) struct DefaultBreakerTransition;

impl BreakerTransition for DefaultBreakerTransition {
    /// Decide whether to admit a new request.  May promote
    /// Open → HalfOpen if the cool-down has elapsed.
    fn admit(&self, req: AdmitRequest) -> Result<AdmitResponse, BreakerDomainError> {
        let mut state = req.state;
        let mut consecutive_successes = req.consecutive_successes;
        let admission = match state {
            BreakerState::Closed => Admission::Proceed,
            BreakerState::HalfOpen => Admission::Proceed,
            BreakerState::Open { since } => {
                if since.elapsed() >= req.config.cool_down() {
                    debug!(
                        cool_down_seconds = req.config.cool_down_seconds,
                        "grpc-breaker: cool-down elapsed, promoting to HalfOpen",
                    );
                    state = BreakerState::HalfOpen;
                    consecutive_successes = 0;
                    Admission::Proceed
                } else {
                    Admission::RejectOpen
                }
            }
        };
        debug!(
            target: BREAKER_TRANSITION_LOG_TARGET,
            decision = Self::describe_admission(admission),
            "grpc-breaker: admission decided",
        );
        Ok(AdmitResponse {
            admission,
            state,
            consecutive_failures: req.consecutive_failures,
            consecutive_successes,
        })
    }

    /// Record the outcome of a dispatched request and update state.
    ///
    /// Called only when [`admit`](Self::admit) returned [`Admission::Proceed`] —
    /// i.e. we actually called the inner client.
    fn record(
        &self,
        req: RecordOutcomeRequest,
    ) -> Result<RecordOutcomeResponse, BreakerDomainError> {
        let mut state = req.state;
        let mut consecutive_failures = req.consecutive_failures;
        let mut consecutive_successes = req.consecutive_successes;
        debug!(
            outcome = Self::describe_outcome(req.outcome),
            "grpc-breaker: recording outcome",
        );
        match (state, req.outcome) {
            (BreakerState::Closed, Outcome::Success) => {
                consecutive_failures = 0;
            }
            (BreakerState::Closed, Outcome::Failure) => {
                consecutive_failures = consecutive_failures.saturating_add(1);
                if consecutive_failures >= req.config.failure_threshold {
                    warn!(
                        failures = consecutive_failures,
                        threshold = req.config.failure_threshold,
                        "grpc-breaker: failure threshold reached, opening",
                    );
                    state = BreakerState::Open {
                        since: Instant::now(),
                    };
                }
            }
            (BreakerState::HalfOpen, Outcome::Success) => {
                consecutive_successes = consecutive_successes.saturating_add(1);
                if consecutive_successes >= req.config.half_open_probe_count {
                    info!(
                        probe_successes = consecutive_successes,
                        "grpc-breaker: probe successful, closing",
                    );
                    state = BreakerState::Closed;
                    consecutive_failures = 0;
                    consecutive_successes = 0;
                }
            }
            (BreakerState::HalfOpen, Outcome::Failure) => {
                warn!("grpc-breaker: probe failed, returning to Open");
                state = BreakerState::Open {
                    since: Instant::now(),
                };
                consecutive_successes = 0;
            }
            (BreakerState::Open { .. }, _) => {
                // record() called while Open — caller should not
                // dispatch in this state.  Ignore defensively.
            }
        }
        Ok(RecordOutcomeResponse {
            state,
            consecutive_failures,
            consecutive_successes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::GrpcBreakerConfig;

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
                state: BreakerState::Closed,
                consecutive_failures: 0,
                consecutive_successes: 0,
                config: cfg(),
            })
            .expect("infallible");
        assert_eq!(resp.admission, Admission::Proceed);
    }

    #[test]
    fn test_record_failure_at_threshold_opens_breaker() {
        let mut state = BreakerState::Closed;
        let mut consecutive_failures = 0;
        let mut consecutive_successes = 0;
        for _ in 0..2 {
            let resp = DefaultBreakerTransition
                .record(RecordOutcomeRequest {
                    state,
                    consecutive_failures,
                    consecutive_successes,
                    config: cfg(),
                    outcome: Outcome::Failure,
                })
                .expect("infallible");
            state = resp.state;
            consecutive_failures = resp.consecutive_failures;
            consecutive_successes = resp.consecutive_successes;
        }
        assert!(matches!(state, BreakerState::Open { .. }));
    }

    #[test]
    fn test_admit_open_node_returns_reject() {
        let resp = DefaultBreakerTransition
            .admit(AdmitRequest {
                state: BreakerState::Open {
                    since: Instant::now(),
                },
                consecutive_failures: 2,
                consecutive_successes: 0,
                config: cfg(),
            })
            .expect("infallible");
        assert_eq!(resp.admission, Admission::RejectOpen);
    }

    #[test]
    fn test_describe_admission_formats_proceed() {
        let s = DefaultBreakerTransition::describe_admission(Admission::Proceed);
        assert_eq!(s, "Proceed");
    }

    #[test]
    fn test_describe_outcome_formats_success() {
        let s = DefaultBreakerTransition::describe_outcome(Outcome::Success);
        assert_eq!(s, "Success");
    }
}
