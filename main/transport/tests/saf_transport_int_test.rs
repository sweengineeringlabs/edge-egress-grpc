//! SAF-level integration tests for `TransportSvc::create_transport_from_config`.

use swe_edge_egress_grpc_transport::{
    GrpcChannelConfig, GrpcChannelConfigError, ResilienceConfig, TransportSvc,
};

fn ensure_rustls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

fn resilience() -> ResilienceConfig {
    ResilienceConfig {
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

/// @covers: TransportSvc::create_transport_from_config — bare transport without resilience.
#[test]
fn transport_struct_transport_create_without_resilience_returns_ok_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    assert!(TransportSvc::create_transport_from_config(&cfg).is_ok());
}

/// @covers: TransportSvc::create_transport_from_config — resilient transport with resilience config.
#[test]
fn transport_struct_transport_create_with_resilience_returns_ok_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(resilience());
    assert!(TransportSvc::create_transport_from_config(&cfg).is_ok());
}

/// @covers: TransportSvc::create_transport_from_config — rejects plaintext when tls_required.
#[test]
fn transport_struct_transport_create_rejects_plaintext_when_tls_required_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051");
    assert!(matches!(
        TransportSvc::create_transport_from_config(&cfg),
        Err(GrpcChannelConfigError::PlaintextRejected(_))
    ));
}
