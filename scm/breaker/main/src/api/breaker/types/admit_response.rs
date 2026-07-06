//! Response for [`crate::api::BreakerTransition::admit`].

use crate::api::Admission;
use crate::api::BreakerState;

/// Output of [`crate::api::BreakerTransition::admit`] — the
/// admission decision and the (possibly promoted) node snapshot,
/// flattened rather than nested — see field_type_purity.
pub struct AdmitResponse {
    /// The admission decision.
    pub admission: Admission,
    /// Breaker state after this decision (possibly promoted).
    pub state: BreakerState,
    /// Consecutive-failure count after this decision.
    pub consecutive_failures: u32,
    /// Consecutive-success count after this decision.
    pub consecutive_successes: u32,
}
