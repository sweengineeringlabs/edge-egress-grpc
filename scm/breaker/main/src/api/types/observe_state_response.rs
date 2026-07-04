//! Response for [`crate::api::traits::BreakerObservable::state`].

use crate::api::types::breaker_state::BreakerState;

/// Output of [`crate::api::traits::BreakerObservable::state`] — a snapshot
/// of the breaker's state; the breaker may transition immediately after
/// this call returns.
pub struct ObserveStateResponse {
    /// The observed breaker state snapshot.
    pub state: BreakerState,
}
