//! Integration tests for `ResilienceValidator` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use swe_edge_egress_grpc_transport::{
    ConfigValidationRequest, GrpcChannelConfigError, ResilienceConfigResilienceValidator,
    ResilienceValidator,
};

/// @covers: ResilienceValidator is object-safe
#[test]
fn transport_trait_resilience_validator_is_object_safe_int_test() {
    fn _assert(_: &dyn ResilienceValidator) {}
}

/// Minimal test-double, used only to exercise `default_config_builder`
/// (a `Self: Sized` default method) from outside the crate.
// @allow: no_mocks_in_integration — hand-rolled test double, not a mock library.
struct StubResilienceValidator;

impl ResilienceValidator for StubResilienceValidator {
    fn validate_config(&self, req: ConfigValidationRequest) -> Result<(), GrpcChannelConfigError> {
        let _ = req;
        Ok(())
    }
}

/// @covers: default_config_builder
#[test]
fn test_default_config_builder_missing_fields_is_error() {
    // A fresh builder has none of ResilienceConfigResilienceValidator's required fields set —
    // `.build()` must reject it, proving this isn't a stub that always
    // succeeds.
    let err = <StubResilienceValidator as ResilienceValidator>::default_config_builder()
        .build()
        .expect_err("an empty resilience config builder must fail to build");
    assert!(!err.is_empty(), "error message must be non-empty");
}

/// @covers: default_config_builder
#[test]
fn test_default_config_builder_fully_populated_builds_valid_config_happy() {
    let config: ResilienceConfigResilienceValidator =
        <StubResilienceValidator as ResilienceValidator>::default_config_builder()
            .max_attempts(3)
            .rate_limit_max_attempts(2)
            .build()
            .expect("a builder with all required fields set must build");
    assert_eq!(config.max_attempts, 3);
    assert_eq!(config.rate_limit_max_attempts, 2);
    // Optional fields must fall back to their documented defaults.
    assert_eq!(config.initial_backoff_ms, 100);
}

/// @covers: default_config_builder
#[test]
fn test_default_config_builder_repeated_calls_are_independent_edge() {
    let a = <StubResilienceValidator as ResilienceValidator>::default_config_builder();
    let b = <StubResilienceValidator as ResilienceValidator>::default_config_builder();
    // Both are fresh, independently-failing builders — mutating one must
    // not affect the other.
    assert_eq!(a.build().is_err(), b.build().is_err());
}

// ── ResilienceValidator::validate_config (real impl: ResilienceConfigResilienceValidator) ──────

fn valid_config() -> ResilienceConfigResilienceValidator {
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

/// @covers: validate_config
#[test]
fn test_validate_config_valid_config_happy() {
    let config = valid_config();
    config
        .validate_config(ConfigValidationRequest {
            config: Arc::new(config.clone()),
        })
        .expect("a fully valid config must be accepted");
}

/// @covers: validate_config
#[test]
fn test_validate_config_zero_max_attempts_error() {
    let mut config = valid_config();
    config.max_attempts = 0;
    let err = config
        .validate_config(ConfigValidationRequest {
            config: Arc::new(config.clone()),
        })
        .expect_err("max_attempts == 0 must be rejected");
    assert!(matches!(err, GrpcChannelConfigError::Config(_)));
}

/// @covers: validate_config
#[test]
fn test_validate_config_ignores_receiver_uses_request_config_edge() {
    // `validate_config` validates `req.config`, not `self` — proven by
    // calling it on a valid receiver with an invalid request config.
    let receiver = valid_config();
    let mut requested = valid_config();
    requested.rate_limit_max_attempts = 0;
    let err = receiver
        .validate_config(ConfigValidationRequest {
            config: Arc::new(requested),
        })
        .expect_err("the request's config must be validated, not the receiver's");
    assert!(matches!(err, GrpcChannelConfigError::Config(_)));
}
