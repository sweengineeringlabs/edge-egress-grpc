//! SAF layer — public facade.

mod resilient_svc;

pub use crate::api::types::GrpcResilientSvc;

pub use crate::api::error::ResilientTransportError;

/// Build a resilient outbound gRPC transport from a [`swe_edge_egress_grpc::GrpcChannelConfig`].
pub fn create_resilient_transport_from_config(
    config: &swe_edge_egress_grpc::GrpcChannelConfig,
) -> Result<std::sync::Arc<dyn swe_edge_egress_grpc::GrpcEgress>, ResilientTransportError> {
    GrpcResilientSvc::create_resilient_transport_from_config(config)
}
