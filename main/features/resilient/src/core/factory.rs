//! Resilient transport assembly: TonicGrpcClient + retry + circuit breaker.

use std::sync::Arc;

use swe_edge_egress_grpc::{
    create_tonic_client_from_config, validate_resilience_config, GrpcChannelConfig, GrpcEgress,
};
use swe_edge_egress_grpc_breaker::{GrpcBreakerClient, GrpcBreakerConfig};
use swe_edge_egress_grpc_retry::{GrpcRetryClient, GrpcRetryConfig};

use crate::api::error::ResilientTransportError;

pub(crate) fn assemble(
    config: &GrpcChannelConfig,
) -> Result<Arc<dyn GrpcEgress>, ResilientTransportError> {
    let base = create_tonic_client_from_config(config)?;

    match &config.resilience {
        None => Ok(Arc::new(base)),
        Some(r) => {
            validate_resilience_config(r).map_err(ResilientTransportError::InvalidResilience)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_egress_grpc::{GrpcChannelConfig, ResilienceConfig};

    fn valid_resilience() -> ResilienceConfig {
        ResilienceConfig {
            max_attempts: 3,
            initial_backoff_ms: 10,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            max_backoff_ms: 100,
            rate_limit_max_attempts: 2,
            rate_limit_initial_backoff_ms: 10,
            rate_limit_max_backoff_ms: 100,
            failure_threshold: 3,
            cool_down_seconds: 10,
            half_open_probe_count: 1,
        }
    }

    fn ensure_tls_provider() {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        });
    }

    /// @covers: assemble — no resilience returns bare transport
    #[test]
    fn test_assemble_without_resilience_returns_ok() {
        ensure_tls_provider();
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
        assert!(assemble(&config).is_ok());
    }

    /// @covers: assemble — with valid resilience config returns ok
    #[test]
    fn test_assemble_with_valid_resilience_returns_ok() {
        ensure_tls_provider();
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
            .allow_plaintext()
            .with_resilience(valid_resilience());
        assert!(assemble(&config).is_ok());
    }

    /// @covers: assemble — TLS required rejects plaintext
    #[test]
    fn test_assemble_tls_required_rejects_plaintext_endpoint() {
        ensure_tls_provider();
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
        let result = assemble(&config);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ResilientTransportError::ChannelConfig(_)
        ));
    }

    /// @covers: assemble — invalid resilience config returns error
    #[test]
    fn test_assemble_invalid_resilience_config_returns_error() {
        ensure_tls_provider();
        let mut r = valid_resilience();
        r.max_attempts = 0;
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
            .allow_plaintext()
            .with_resilience(r);
        let result = assemble(&config);
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ResilientTransportError::InvalidResilience(_)
        ));
    }
}
