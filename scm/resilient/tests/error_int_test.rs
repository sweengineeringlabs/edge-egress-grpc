#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `ResilientTransportError`.

use swe_edge_egress_grpc::GrpcChannelConfig;
use swe_edge_egress_grpc_resilient::{GrpcResilientFacade, ResilientTransportError};

/// @covers: ResilientTransportError::ChannelConfig
#[test]
fn test_error_channel_config_variant_produced_on_plaintext_rejection() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
    let err = GrpcResilientFacade::create_resilient_transport_from_config(&config)
        .err()
        .unwrap();
    assert!(matches!(err, ResilientTransportError::ChannelConfig(_)));
    assert!(!err.to_string().is_empty());
}

/// @covers: ResilientTransportError::InvalidResilience
#[test]
fn test_error_invalid_resilience_variant_produced_on_bad_config() {
    use swe_edge_egress_grpc::ResilienceConfigResilienceValidator;
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();
    let r = ResilienceConfigResilienceValidator {
        max_attempts: 0,
        ..ResilienceConfigResilienceValidator::default()
    };
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(r);
    let err = GrpcResilientFacade::create_resilient_transport_from_config(&config)
        .err()
        .unwrap();
    assert!(matches!(err, ResilientTransportError::InvalidResilience(_)));
    assert!(err.to_string().contains("invalid resilience config"));
}
