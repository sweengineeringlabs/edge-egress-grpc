//! gRPC resilient SAF — factory methods on [`GrpcResilientSvc`].

use std::sync::Arc;

use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress, TransportSvc};
use swe_edge_egress_grpc_breaker::{GrpcBreakerClient, GrpcBreakerConfig};
use swe_edge_egress_grpc_retry::{GrpcRetryClient, GrpcRetryConfig};

pub use crate::api::error::resilient_transport_error::ResilientTransportError;
pub use crate::api::types::grpc_resilient_svc::GrpcResilientSvc;

pub use crate::api::ApplicationConfigBuilder;

impl GrpcResilientSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a resilient outbound gRPC transport from a [`GrpcChannelConfig`].
    ///
    /// When `config.resilience` is `Some`, wraps the base transport in a
    /// [`GrpcRetryClient`] then a [`GrpcBreakerClient`].
    pub fn create_resilient_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, ResilientTransportError> {
        let base = TransportSvc::create_tonic_client_from_config(config)?;

        match &config.resilience {
            None => Ok(Arc::new(base)),
            Some(r) => {
                TransportSvc::validate_resilience_config(r)
                    .map_err(ResilientTransportError::InvalidResilience)?;

                let retry_cfg = GrpcRetryConfig {
                    max_attempts: r.max_attempts,
                    initial_backoff_ms: r.initial_backoff_ms,
                    backoff_multiplier: r.backoff_multiplier,
                    jitter_factor: r.jitter_factor,
                    max_backoff_ms: r.max_backoff_ms,
                    rate_limit_max_attempts: r.rate_limit_max_attempts,
                    rate_limit_initial_backoff_ms: r.rate_limit_initial_backoff_ms,
                    rate_limit_max_backoff_ms: r.rate_limit_max_backoff_ms,
                };
                let breaker_cfg = GrpcBreakerConfig {
                    failure_threshold: r.failure_threshold,
                    cool_down_seconds: r.cool_down_seconds,
                    half_open_probe_count: r.half_open_probe_count,
                };

                let with_retry = GrpcRetryClient::new(base, retry_cfg);
                let with_breaker = GrpcBreakerClient::new(with_retry, breaker_cfg);
                Ok(Arc::new(with_breaker))
            }
        }
    }
}
