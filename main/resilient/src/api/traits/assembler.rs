//! Factory contracts — interface types for the resilient transport assembler.

use std::sync::Arc;

use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress};

use crate::api::error::resilient_transport_error::ResilientTransportError;

/// Contract for assembling a resilient gRPC transport stack.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor (Rule 121) — intentionally unused"
)]
pub trait Assembler {
    /// Build a resilient `GrpcEgress` from the channel configuration.
    fn assemble(config: &GrpcChannelConfig)
        -> Result<Arc<dyn GrpcEgress>, ResilientTransportError>;
}
