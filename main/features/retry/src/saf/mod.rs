//! SAF layer — public facade.

mod retry_svc;

pub use crate::api::types::{GrpcRetryClient, GrpcRetrySvc};

pub use crate::api::backoff::BackoffSchedule;
pub use crate::api::error::Error;
pub use crate::api::retry_config::GrpcRetryConfig;
pub use crate::api::retry_policy::{
    classify, classify_resource_exhausted, parse_retry_after_hint, ResourceExhaustedContext,
    RetryDecision,
};

/// Return a config builder pre-seeded with this crate's name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    GrpcRetrySvc::create_config_builder()
}

/// Wrap `inner` with the supplied retry policy.
pub fn wrap_retry<T: swe_edge_egress_grpc::GrpcEgress + Send + Sync + 'static>(
    inner: T,
    config: GrpcRetryConfig,
) -> GrpcRetryClient<T> {
    GrpcRetrySvc::wrap_retry(inner, config)
}

/// Wrap `inner` with the default retry policy.
pub fn create_retry_client<T: swe_edge_egress_grpc::GrpcEgress + Send + Sync + 'static>(
    inner: T,
) -> GrpcRetryClient<T> {
    GrpcRetrySvc::create_retry_client(inner)
}
