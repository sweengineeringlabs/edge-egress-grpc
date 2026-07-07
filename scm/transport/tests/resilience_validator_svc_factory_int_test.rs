//! Integration tests for `ResilienceValidatorFactory`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use swe_edge_egress_grpc_transport::{
    ConfigValidationRequest, ResilienceConfigResilienceValidator, ResilienceValidatorFactory,
};

/// @covers: create
#[test]
fn test_create_returns_validator_accepting_default_config_happy() {
    let validator = ResilienceValidatorFactory::create();
    let req = ConfigValidationRequest {
        config: Arc::new(ResilienceConfigResilienceValidator::default()),
    };
    validator
        .validate_config(req)
        .expect("the default resilience config must validate cleanly");
}

/// @covers: create
#[test]
fn test_create_each_call_returns_independent_instance_edge() {
    let a = ResilienceValidatorFactory::create();
    let b = ResilienceValidatorFactory::create();
    let valid_req = ConfigValidationRequest {
        config: Arc::new(ResilienceConfigResilienceValidator::default()),
    };
    let invalid_fields = ResilienceConfigResilienceValidator {
        max_attempts: 0,
        ..ResilienceConfigResilienceValidator::default()
    };
    let invalid_req = ConfigValidationRequest {
        config: Arc::new(invalid_fields),
    };
    // Two independently-created validators must judge each request on its
    // own merits — one valid, one invalid — not share hidden state.
    assert!(a.validate_config(valid_req).is_ok());
    assert!(b.validate_config(invalid_req).is_err());
}

/// @covers: create
#[test]
fn test_create_rejects_invalid_config_edge() {
    let validator = ResilienceValidatorFactory::create();
    let invalid = ResilienceConfigResilienceValidator {
        max_attempts: 0,
        ..ResilienceConfigResilienceValidator::default()
    };
    let req = ConfigValidationRequest {
        config: Arc::new(invalid),
    };
    assert!(
        validator.validate_config(req).is_err(),
        "an invalid config must be rejected, proving this isn't a stub that always says Ok"
    );
}
