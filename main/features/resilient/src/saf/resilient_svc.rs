//! gRPC resilient SAF — factory methods on [`GrpcResilientSvc`].

use std::sync::Arc;

use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress};

use crate::api::error::ResilientTransportError;
use crate::api::types::resilient_svc::GrpcResilientSvc;

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
        let transport = crate::core::factory::assemble(config)?;
        Ok(transport)
    }
}
