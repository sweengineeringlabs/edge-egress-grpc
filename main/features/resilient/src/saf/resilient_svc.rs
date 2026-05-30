//! gRPC resilient SAF — factory methods on [`GrpcResilientSvc`].

use std::sync::Arc;

use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress};

use crate::api::error::resilient_transport_error::ResilientTransportError;
use crate::api::types::grpc_resilient_svc::GrpcResilientSvc;
use crate::core::factory::ResilientAssembler;

impl GrpcResilientSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build a resilient outbound gRPC transport from a [`GrpcChannelConfig`].
    pub fn create_resilient_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, ResilientTransportError> {
        ResilientAssembler::assemble(config)
    }
}
