//! gRPC retry SAF — factory methods on [`GrpcRetrySvc`].

use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_edge_egress_grpc::GrpcEgress;

use crate::api::types::grpc_retry_client::GrpcRetryClient;
use crate::api::types::grpc_retry_config::GrpcRetryConfig;
use crate::api::types::grpc_retry_svc::GrpcRetrySvc;

impl GrpcRetrySvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigBuilderImpl::for_crate(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    }

    /// Wrap `inner` with the supplied retry policy.
    pub fn wrap_retry<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
        config: GrpcRetryConfig,
    ) -> GrpcRetryClient<T> {
        GrpcRetryClient {
            inner,
            config: std::sync::Arc::new(config),
        }
    }

    /// Wrap `inner` with the default retry policy.
    pub fn create_retry_client<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
    ) -> GrpcRetryClient<T> {
        GrpcRetryClient {
            inner,
            config: std::sync::Arc::new(GrpcRetryConfig::default()),
        }
    }
}
