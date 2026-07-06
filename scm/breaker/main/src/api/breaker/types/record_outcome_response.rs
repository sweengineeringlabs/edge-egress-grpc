//! Response for [`crate::api::BreakerTransition::record`].

use crate::api::BreakerState;

/// Output of [`crate::api::BreakerTransition::record`] — the node
/// snapshot after applying the recorded outcome, flattened rather than
/// nested — see field_type_purity.
pub struct RecordOutcomeResponse {
    /// Breaker state after applying the outcome.
    pub state: BreakerState,
    /// Consecutive-failure count after applying the outcome.
    pub consecutive_failures: u32,
    /// Consecutive-success count after applying the outcome.
    pub consecutive_successes: u32,
}
