//! gRPC retry SAF — factory methods on [`GrpcRetrySvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_edge_egress_grpc::GrpcEgress;

use crate::api::retry_config::GrpcRetryConfig;
use crate::api::types::retry_svc::GrpcRetrySvc;
use crate::api::types::GrpcRetryClient;

impl GrpcRetrySvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let builder = ConfigLoaderFactory::create_config_builder();
        builder
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Wrap `inner` with the supplied retry policy.
    pub fn wrap_retry<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
        config: GrpcRetryConfig,
    ) -> GrpcRetryClient<T> {
        let client = GrpcRetryClient::new(inner, config);
        client
    }

    /// Wrap `inner` with the default retry policy.
    pub fn create_retry_client<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
    ) -> GrpcRetryClient<T> {
        let client = GrpcRetryClient::new(inner, GrpcRetryConfig::default());
        client
    }
}
