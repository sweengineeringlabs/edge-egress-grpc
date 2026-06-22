//! gRPC retry SAF — factory methods on [`GrpcRetrySvc`].

use swe_edge_egress_grpc::GrpcEgress;

pub use crate::api::types::grpc_retry_client::GrpcRetryClient;
pub use crate::api::types::grpc_retry_config::GrpcRetryConfig;
pub use crate::api::types::grpc_retry_svc::GrpcRetrySvc;

pub use crate::api::error::Error;
pub use crate::api::types::{
    BackoffSchedule, GrpcRetryConfigBuilder, ResourceExhaustedContext, RetryDecision,
};

impl GrpcRetrySvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
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
