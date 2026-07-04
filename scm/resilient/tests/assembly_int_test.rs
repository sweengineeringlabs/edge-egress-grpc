#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `GrpcResilientSvc::create_resilient_transport_from_config`.

use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgressError, ResilienceConfig};
use swe_edge_egress_grpc_resilient::{GrpcResilientSvc, ResilientTransportError};

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
    static ONCE: Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[tokio::test]
async fn test_without_resilience_returns_ok() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let transport = GrpcResilientSvc::create_resilient_transport_from_config(&config)
        .expect("assembly must succeed for a valid plaintext config");
    // Nothing listens on 127.0.0.1:50051 in the test environment, so a real
    // call must genuinely fail — proves this is a connectable client, not a stub.
    let health = transport.health_check().await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[tokio::test]
async fn test_with_valid_resilience_returns_ok() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(valid_resilience());
    let transport = GrpcResilientSvc::create_resilient_transport_from_config(&config)
        .expect("assembly must succeed for a valid resilience config");
    let health = transport.health_check().await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[test]
fn test_tls_required_rejects_plaintext_endpoint() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
    let result = GrpcResilientSvc::create_resilient_transport_from_config(&config);
    assert!(result.is_err());
    assert!(matches!(
        result.err().unwrap(),
        ResilientTransportError::ChannelConfig(_)
    ));
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[test]
fn test_invalid_resilience_config_returns_error() {
    ensure_tls_provider();
    let mut r = valid_resilience();
    r.max_attempts = 0;
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(r);
    let result = GrpcResilientSvc::create_resilient_transport_from_config(&config);
    assert!(result.is_err());
    assert!(matches!(
        result.err().unwrap(),
        ResilientTransportError::InvalidResilience(_)
    ));
}
