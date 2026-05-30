//! Factory contracts — interface types for the resilient transport assembler.

use std::sync::Arc;

use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress};

use crate::api::error::resilient_transport_error::ResilientTransportError;

/// Contract for assembling a resilient gRPC transport stack.
pub(crate) trait Assembler {
    /// Build a resilient `GrpcEgress` from the channel configuration.
    fn assemble(config: &GrpcChannelConfig)
        -> Result<Arc<dyn GrpcEgress>, ResilientTransportError>;
}
