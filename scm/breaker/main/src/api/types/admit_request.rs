//! Request for [`crate::api::traits::BreakerTransition::admit`].

use crate::api::types::breaker_node::BreakerNode;
use crate::api::types::grpc_breaker_config::GrpcBreakerConfig;

/// Input to [`crate::api::traits::BreakerTransition::admit`] — the current
/// node snapshot and the active policy to evaluate it against.
pub(crate) struct AdmitRequest {
    pub(crate) node: BreakerNode,
    pub(crate) config: GrpcBreakerConfig,
}
