//! Factory functions — build outbound transports from config.

use std::sync::Arc;
use std::time::Duration;

use crate::api::port::{GrpcChannelConfigError, GrpcOutbound};
use crate::api::value_object::ResilienceConfig;
use crate::core::client::tonic_grpc_client::TonicGrpcClient;
use crate::core::resilience::circuit_breaker::CircuitBreaker;
use crate::core::resilience::resilient_grpc_client::ResilientGrpcClient;
use crate::core::resilience::retry::RetryPolicy;

/// Build an outbound transport from a [`crate::api::value_object::GrpcChannelConfig`].
///
/// When `config.resilience` is `Some`, the bare [`TonicGrpcClient`] is
/// automatically composed with [`ResilientGrpcClient`] (retry + circuit breaker).
/// When `config.resilience` is `None`, the bare transport is returned.
///
/// # Errors
///
/// Returns [`GrpcChannelConfigError::PlaintextRejected`] when
/// `config.tls_required = true` and the endpoint scheme is `http://`.
/// Returns [`GrpcChannelConfigError::Config`] when the resilience config
/// fails validation.
pub fn create_transport_from_config(
    config: &crate::api::value_object::GrpcChannelConfig,
) -> Result<Arc<dyn GrpcOutbound>, GrpcChannelConfigError> {
    let base: Arc<dyn GrpcOutbound> = Arc::new(TonicGrpcClient::from_config(config)?);

    Ok(match &config.resilience {
        None => base,
        Some(r) => {
            let retry   = build_retry_policy(r)?;
            let breaker = build_circuit_breaker(r)?;
            Arc::new(ResilientGrpcClient::new(base, retry, breaker))
        }
    })
}

fn build_retry_policy(r: &ResilienceConfig) -> Result<RetryPolicy, GrpcChannelConfigError> {
    if r.max_attempts == 0 {
        return Err(GrpcChannelConfigError::Config("max_attempts must be >= 1".into()));
    }
    if r.rate_limit_max_attempts == 0 {
        return Err(GrpcChannelConfigError::Config("rate_limit_max_attempts must be >= 1".into()));
    }
    if r.jitter_factor < 0.0 || r.jitter_factor > 1.0 {
        return Err(GrpcChannelConfigError::Config("jitter_factor must be in [0.0, 1.0]".into()));
    }
    Ok(RetryPolicy {
        max_attempts:               r.max_attempts,
        initial_backoff:            Duration::from_millis(r.initial_backoff_ms),
        backoff_multiplier:         r.backoff_multiplier,
        jitter_factor:              r.jitter_factor,
        max_backoff:                Duration::from_millis(r.max_backoff_ms),
        rate_limit_max_attempts:    r.rate_limit_max_attempts,
        rate_limit_initial_backoff: Duration::from_millis(r.rate_limit_initial_backoff_ms),
        rate_limit_max_backoff:     Duration::from_millis(r.rate_limit_max_backoff_ms),
    })
}

fn build_circuit_breaker(r: &ResilienceConfig) -> Result<CircuitBreaker, GrpcChannelConfigError> {
    if r.half_open_probe_count == 0 {
        return Err(GrpcChannelConfigError::Config("half_open_probe_count must be >= 1".into()));
    }
    Ok(CircuitBreaker::new(
        r.failure_threshold,
        Duration::from_secs(r.cool_down_seconds),
        r.half_open_probe_count,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::{GrpcChannelConfig, ResilienceConfig};

    fn resilience() -> ResilienceConfig {
        ResilienceConfig {
            max_attempts:                  3,
            initial_backoff_ms:            10,
            backoff_multiplier:            2.0,
            jitter_factor:                 0.1,
            max_backoff_ms:                1000,
            failure_threshold:             5,
            cool_down_seconds:             30,
            half_open_probe_count:         1,
            rate_limit_max_attempts:       2,
            rate_limit_initial_backoff_ms: 500,
            rate_limit_max_backoff_ms:     5000,
        }
    }

    /// @covers: create_transport_from_config
    #[test]
    fn test_create_transport_from_config_without_resilience_returns_ok() {
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
        assert!(create_transport_from_config(&config).is_ok());
    }

    /// @covers: create_transport_from_config
    #[test]
    fn test_create_transport_from_config_with_resilience_returns_ok() {
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
            .allow_plaintext()
            .with_resilience(resilience());
        assert!(create_transport_from_config(&config).is_ok());
    }

    /// @covers: create_transport_from_config
    #[test]
    fn test_build_retry_policy_rejects_zero_max_attempts() {
        let mut r = resilience();
        r.max_attempts = 0;
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
            .allow_plaintext()
            .with_resilience(r);
        assert!(matches!(
            create_transport_from_config(&config),
            Err(GrpcChannelConfigError::Config(_))
        ));
    }

    /// @covers: create_transport_from_config
    #[test]
    fn test_build_circuit_breaker_rejects_zero_probe_count() {
        let mut r = resilience();
        r.half_open_probe_count = 0;
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
            .allow_plaintext()
            .with_resilience(r);
        assert!(matches!(
            create_transport_from_config(&config),
            Err(GrpcChannelConfigError::Config(_))
        ));
    }
}
