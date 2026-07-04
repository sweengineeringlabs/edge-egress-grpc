//! Response for [`crate::api::traits::BreakerTransition::record`].

use crate::api::types::breaker_node::BreakerNode;

/// Output of [`crate::api::traits::BreakerTransition::record`] — the
/// node snapshot after applying the recorded outcome.
pub(crate) struct RecordOutcomeResponse {
    pub(crate) node: BreakerNode,
}
