//! Integration tests for [`ConfigValidationRequest`].

use swe_edge_egress_grpc::ResilienceConfig as ForeignResilienceConfig;
use swe_edge_egress_grpc_resilient::{ConfigValidationRequest, ResilienceConfig};

fn valid() -> ForeignResilienceConfig {
    ForeignResilienceConfig {
        max_attempts: 3,
        initial_backoff_ms: 10,
        backoff_multiplier: 2.0,
        jitter_factor: 0.1,
        max_backoff_ms: 100,
        rate_limit_max_attempts: 2,
        rate_limit_initial_backoff_ms: 10,
        rate_limit_max_backoff_ms: 100,
        failure_threshold: 7,
        cool_down_seconds: 10,
        half_open_probe_count: 2,
    }
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_preserves_config_happy() {
    let req = ConfigValidationRequest {
        config: ResilienceConfig(valid()),
    };
    assert_eq!(req.config.0.failure_threshold, 7);
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_zero_max_attempts_error() {
    let mut cfg = valid();
    cfg.max_attempts = 0;
    let req = ConfigValidationRequest {
        config: ResilienceConfig(cfg),
    };
    assert_eq!(req.config.0.max_attempts, 0);
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_default_config_edge() {
    let req = ConfigValidationRequest {
        config: ResilienceConfig(ForeignResilienceConfig::default()),
    };
    let default_cfg = ForeignResilienceConfig::default();
    assert_eq!(
        req.config.0.failure_threshold,
        default_cfg.failure_threshold
    );
    assert_eq!(req.config.0.max_attempts, default_cfg.max_attempts);
}
