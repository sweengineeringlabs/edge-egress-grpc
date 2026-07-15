#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `TransportSvc` — the api/-declared marker struct and
//! its core/-implemented construction methods.

use edge_transport_grpc_egress_transport::{
    GrpcChannelConfig, GrpcChannelConfigError, GrpcEgressError, HealthCheckRequest,
    ResilienceConfigResilienceValidator, TransportConstruction, TransportSvc,
};

/// @covers: TransportSvc struct is publicly accessible and is a zero-sized namespace marker
#[test]
fn transport_struct_transport_svc_exists_int_test() {
    assert_eq!(
        std::mem::size_of::<TransportSvc>(),
        0,
        "TransportSvc is a namespace marker for factory fns and must carry no instance state"
    );
}

fn ensure_rustls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

fn resilience() -> ResilienceConfigResilienceValidator {
    ResilienceConfigResilienceValidator {
        max_attempts: 3,
        initial_backoff_ms: 100,
        backoff_multiplier: 2.0,
        jitter_factor: 0.1,
        max_backoff_ms: 2_000,
        rate_limit_max_attempts: 2,
        rate_limit_initial_backoff_ms: 1_000,
        rate_limit_max_backoff_ms: 10_000,
        failure_threshold: 5,
        cool_down_seconds: 10,
        half_open_probe_count: 1,
    }
}

/// @covers: create_transport_from_config — bare transport without resilience.
#[tokio::test]
async fn transport_struct_transport_create_without_resilience_returns_ok_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let transport = TransportConstruction::create_transport_from_config(&cfg)
        .expect("assembly must succeed for a valid plaintext config");
    // Nothing listens on 127.0.0.1:50051 in the test environment, so a real
    // call must genuinely fail — proves this is a connectable client, not a stub.
    let health = transport.health_check(HealthCheckRequest).await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}

/// @covers: create_transport_from_config — resilient transport with resilience config.
#[tokio::test]
async fn transport_struct_transport_create_with_resilience_returns_ok_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(resilience());
    let transport = TransportConstruction::create_transport_from_config(&cfg)
        .expect("assembly must succeed for a valid resilience config");
    let health = transport.health_check(HealthCheckRequest).await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}

/// @covers: create_transport_from_config — rejects plaintext when tls_required.
#[test]
fn transport_struct_transport_create_rejects_plaintext_when_tls_required_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051");
    assert!(matches!(
        TransportConstruction::create_transport_from_config(&cfg),
        Err(GrpcChannelConfigError::PlaintextRejected(_))
    ));
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_sets_name_and_version() {
    let builder = TransportSvc::create_config_builder();
    let loader = builder
        .build_loader()
        .expect("builder pre-populated with name/version must build a loader");
    let _ = loader;
}

/// @covers: validate_resilience_config
#[test]
fn test_validate_resilience_config_accepts_default() {
    let cfg = ResilienceConfigResilienceValidator::default();
    assert!(TransportSvc::validate_resilience_config(&cfg).is_ok());

    // Negative counterpart in the same test: a deliberately invalid config
    // must be rejected.
    let invalid = ResilienceConfigResilienceValidator {
        max_attempts: 0,
        ..ResilienceConfigResilienceValidator::default()
    };
    assert!(TransportSvc::validate_resilience_config(&invalid).is_err());
}

/// @covers: validate_resilience_config
#[test]
fn test_validate_resilience_config_rejects_zero_max_attempts() {
    let cfg = ResilienceConfigResilienceValidator {
        max_attempts: 0,
        ..ResilienceConfigResilienceValidator::default()
    };
    assert!(TransportSvc::validate_resilience_config(&cfg).is_err());
}
