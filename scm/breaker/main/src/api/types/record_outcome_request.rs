//! Request for [`crate::api::traits::BreakerTransition::record`].

use crate::api::types::breaker_node::BreakerNode;
use crate::api::types::grpc_breaker_config::GrpcBreakerConfig;
use crate::api::types::outcome::Outcome;

/// Input to [`crate::api::traits::BreakerTransition::record`] — the node
/// snapshot to update, the active policy, and the outcome to apply.
pub(crate) struct RecordOutcomeRequest {
    pub(crate) node: BreakerNode,
    pub(crate) config: GrpcBreakerConfig,
    pub(crate) outcome: Outcome,
}
