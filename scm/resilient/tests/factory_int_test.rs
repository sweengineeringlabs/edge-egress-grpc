//! Integration tests for `GrpcResilientSvc::create_resilient_transport_from_config`.

use swe_edge_egress_grpc::{GrpcChannelConfig, ResilienceConfig};
use swe_edge_egress_grpc_resilient::GrpcResilientSvc;

fn valid_resilience() -> ResilienceConfig {
    ResilienceConfig::default()
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[test]
fn test_factory_without_resilience_returns_ok() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    assert!(GrpcResilientSvc::create_resilient_transport_from_config(&config).is_ok());
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[test]
fn test_factory_with_valid_resilience_returns_ok() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(valid_resilience());
    assert!(GrpcResilientSvc::create_resilient_transport_from_config(&config).is_ok());
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[test]
fn test_factory_tls_required_rejects_plaintext_endpoint() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
    assert!(GrpcResilientSvc::create_resilient_transport_from_config(&config).is_err());
}
