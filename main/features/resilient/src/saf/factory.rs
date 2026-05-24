//! Public factory for the resilient gRPC transport.

use swe_edge_configbuilder::ConfigBuilder as _;
use std::sync::Arc;

use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress};

use crate::api::error::ResilientTransportError;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build a resilient outbound gRPC transport from a [`GrpcChannelConfig`].
///
/// When `config.resilience` is `Some`, the returned transport wraps
/// [`swe_edge_egress_grpc::TonicGrpcClient`] in a
/// [`swe_edge_egress_grpc_retry::GrpcRetryClient`] (retry layer) then a
/// [`swe_edge_egress_grpc_breaker::GrpcBreakerClient`] (circuit-breaker layer).
///
/// When `config.resilience` is `None`, returns a bare `TonicGrpcClient`.
///
/// ## Errors
///
/// - [`ResilientTransportError::ChannelConfig`] — `tls_required = true` and
///   endpoint uses `http://`.
/// - [`ResilientTransportError::InvalidResilience`] — resilience policy fails
///   validation (e.g. `max_attempts = 0`).
pub fn create_resilient_transport_from_config(
    config: &GrpcChannelConfig,
) -> Result<Arc<dyn GrpcEgress>, ResilientTransportError> {
    crate::core::factory::assemble(config)
}
