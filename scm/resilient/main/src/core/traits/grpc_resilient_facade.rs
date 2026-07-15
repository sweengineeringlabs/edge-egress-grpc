//! `impl GrpcResilientFacade` — composes this crate's default trait
//! implementations directly (no saf/ dependency, keeping core/ → saf/
//! import-free per the SEA dependency direction).

use std::sync::Arc;

use edge_transport_grpc_egress::{GrpcChannelConfig, GrpcEgress, TransportConstruction};
use edge_transport_grpc_egress_breaker::{GrpcBreakerClient, GrpcBreakerConfig};
use edge_transport_grpc_egress_retry::{GrpcRetryClient, GrpcRetryConfig};

use crate::api::{
    ApplicationConfigBuilder, ConfigBuilderProvider, ConfigBuilderRequest, ConfigValidationRequest,
    GrpcResilientFacade, GrpcResilientSvcProcessor, ResilienceConfig, ResilientTransportError,
    Validator,
};
use crate::core::traits::default_validator::DefaultValidator;

impl GrpcResilientFacade {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> Result<ApplicationConfigBuilder, ResilientTransportError> {
        Ok(GrpcResilientSvcProcessor
            .create_config_builder(ConfigBuilderRequest)?
            .builder)
    }

    /// Build a resilient outbound gRPC transport from a [`GrpcChannelConfig`].
    ///
    /// When `config.resilience` is `Some`, wraps the base transport in a
    /// [`GrpcRetryClient`] then a [`GrpcBreakerClient`].
    pub fn create_resilient_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, ResilientTransportError> {
        let base = TransportConstruction::create_tonic_client_from_config(config)?;

        match &config.resilience {
            None => Ok(Arc::new(base)),
            Some(r) => {
                DefaultValidator.validate(ConfigValidationRequest {
                    config: ResilienceConfig(r.clone()),
                })?;

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
