//! Integration tests for `create_resilient_transport_from_config`.

use swe_edge_egress_grpc::{GrpcChannelConfig, ResilienceConfig};
use swe_edge_egress_grpc_resilient::create_resilient_transport_from_config;

fn valid_resilience() -> ResilienceConfig {
    ResilienceConfig::default()
}

/// @covers: create_resilient_transport_from_config — no resilience returns bare transport
#[test]
fn test_factory_without_resilience_returns_ok() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    assert!(create_resilient_transport_from_config(&config).is_ok());
}

/// @covers: create_resilient_transport_from_config — with valid resilience returns wrapped transport
#[test]
fn test_factory_with_valid_resilience_returns_ok() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(valid_resilience());
    assert!(create_resilient_transport_from_config(&config).is_ok());
}

/// @covers: create_resilient_transport_from_config — TLS required rejects plaintext
#[test]
fn test_factory_tls_required_rejects_plaintext_endpoint() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
    assert!(create_resilient_transport_from_config(&config).is_err());
}
