#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `GrpcResilientSvc::create_resilient_transport_from_config`.

use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgressError, ResilienceConfig};
use swe_edge_egress_grpc_resilient::GrpcResilientSvc;

fn valid_resilience() -> ResilienceConfig {
    ResilienceConfig::default()
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[tokio::test]
async fn test_factory_without_resilience_returns_ok() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
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
async fn test_factory_with_valid_resilience_returns_ok() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
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
fn test_factory_tls_required_rejects_plaintext_endpoint() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
    assert!(GrpcResilientSvc::create_resilient_transport_from_config(&config).is_err());
}
