//! gRPC auth-bearer SAF — factory methods on [`GrpcAuthBearerSvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;

/// gRPC bearer-auth building-block factory.
pub struct GrpcAuthBearerSvc;

impl GrpcAuthBearerSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }
}
