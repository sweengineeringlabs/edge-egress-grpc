//! Request for [`crate::api::BreakerTransition::admit`].

use crate::api::BreakerState;
use crate::api::GrpcBreakerConfig;

/// Input to [`crate::api::BreakerTransition::admit`] — the current
/// node snapshot (flattened, not nested — see field_type_purity) and the
/// active policy to evaluate it against.
pub struct AdmitRequest {
    /// Current breaker state.
    pub state: BreakerState,
    /// Current consecutive-failure count.
    pub consecutive_failures: u32,
    /// Current consecutive-success count.
    pub consecutive_successes: u32,
    /// The active breaker policy.
    pub config: GrpcBreakerConfig,
}
