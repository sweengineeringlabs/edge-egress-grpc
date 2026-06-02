//! gRPC breaker SAF — factory methods on [`GrpcBreakerSvc`].

use swe_edge_egress_grpc::GrpcEgress;

use crate::api::breaker::GrpcBreakerConfig;
use crate::api::types::grpc::grpc_breaker_svc::GrpcBreakerSvc;
use crate::api::types::GrpcBreakerClient;

impl GrpcBreakerSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Wrap `inner` with the supplied breaker policy.
    pub fn wrap_breaker<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
        config: GrpcBreakerConfig,
    ) -> GrpcBreakerClient<T> {
        GrpcBreakerClient {
            inner,
            config: std::sync::Arc::new(config),
            node: std::sync::Arc::new(tokio::sync::Mutex::new(
                crate::api::breaker::node::BreakerNode::new(),
            )),
        }
    }

    /// Wrap `inner` with the default breaker policy.
    pub fn create_breaker_client<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
    ) -> GrpcBreakerClient<T> {
        GrpcBreakerClient {
            inner,
            config: std::sync::Arc::new(GrpcBreakerConfig::default()),
            node: std::sync::Arc::new(tokio::sync::Mutex::new(
                crate::api::breaker::node::BreakerNode::new(),
            )),
        }
    }
}
