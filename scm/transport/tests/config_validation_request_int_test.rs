//! Integration tests for `ConfigValidationRequest`.

use swe_edge_egress_grpc_transport::{ConfigValidationRequest, ResilienceConfig};

fn sample() -> ResilienceConfig {
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

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_carries_config_happy() {
    let req = ConfigValidationRequest { config: sample() };
    assert_eq!(req.config.max_attempts, 3);
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_zero_max_attempts_error() {
    let mut config = sample();
    config.max_attempts = 0;
    let req = ConfigValidationRequest { config };
    assert_eq!(req.config.max_attempts, 0);
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_clone_is_independent_edge() {
    let req = ConfigValidationRequest { config: sample() };
    let mut cloned = req.clone();
    cloned.config.max_attempts = 99;
    assert_eq!(
        req.config.max_attempts, 3,
        "cloning must not alias the original"
    );
    assert_eq!(cloned.config.max_attempts, 99);
}
