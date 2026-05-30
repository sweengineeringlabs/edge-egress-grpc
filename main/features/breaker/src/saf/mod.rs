//! SAF layer — public facade.

mod breaker_svc;

pub use crate::api::types::{GrpcBreakerClient, GrpcBreakerSvc};

pub use crate::api::breaker_config::GrpcBreakerConfig;
pub use crate::api::breaker_state::BreakerState;
pub use crate::api::error::Error;

/// Return a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    GrpcBreakerSvc::create_config_builder()
}

/// Wrap `inner` with the supplied breaker policy.
pub fn wrap_breaker<T: swe_edge_egress_grpc::GrpcEgress + Send + Sync + 'static>(
    inner: T,
    config: GrpcBreakerConfig,
) -> GrpcBreakerClient<T> {
    GrpcBreakerSvc::wrap_breaker(inner, config)
}

/// Wrap `inner` with the default breaker policy.
pub fn create_breaker_client<T: swe_edge_egress_grpc::GrpcEgress + Send + Sync + 'static>(
    inner: T,
) -> GrpcBreakerClient<T> {
    GrpcBreakerSvc::create_breaker_client(inner)
}
