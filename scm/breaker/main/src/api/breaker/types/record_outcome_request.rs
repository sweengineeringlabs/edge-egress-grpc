//! Request for [`crate::api::BreakerTransition::record`].

use crate::api::BreakerState;
use crate::api::GrpcBreakerConfig;
use crate::api::Outcome;

// @allow: suggest_builder_pattern — constructed directly in exactly one
// production call site (core/breaker/breaker_egress.rs) plus tests; a
// fluent builder would add indirection with no real caller benefit.
/// Input to [`crate::api::BreakerTransition::record`] — the node
/// snapshot to update (flattened, not nested — see field_type_purity),
/// the active policy, and the outcome to apply.
pub struct RecordOutcomeRequest {
    /// Current breaker state.
    pub state: BreakerState,
    /// Current consecutive-failure count.
    pub consecutive_failures: u32,
    /// Current consecutive-success count.
    pub consecutive_successes: u32,
    /// The active breaker policy.
    pub config: GrpcBreakerConfig,
    /// The outcome to apply.
    pub outcome: Outcome,
}
