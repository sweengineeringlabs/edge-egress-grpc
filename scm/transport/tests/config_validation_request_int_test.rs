//! Integration tests for `ConfigValidationRequest`.

use std::sync::Arc;

use edge_transport_grpc_egress_transport::{
    ConfigValidationRequest, ResilienceConfigResilienceValidator, ValidationRequest, Validator,
};

fn valid_fields() -> ResilienceConfigResilienceValidator {
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

fn sample() -> Arc<dyn Validator> {
    Arc::new(valid_fields())
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_carries_config_happy() {
    let req = ConfigValidationRequest { config: sample() };
    assert!(
        req.config.validate(ValidationRequest).is_ok(),
        "a fully valid config carried by the request must validate cleanly"
    );

    // Negative counterpart in the same test: an invalid config carried the
    // same way must fail — proves `config` isn't a stub that always says Ok.
    let mut invalid_fields = valid_fields();
    invalid_fields.max_attempts = 0;
    let invalid_req = ConfigValidationRequest {
        config: Arc::new(invalid_fields),
    };
    assert!(
        invalid_req.config.validate(ValidationRequest).is_err(),
        "an invalid config carried the same way must fail validation"
    );
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_carries_config_error() {
    let mut fields = valid_fields();
    fields.max_attempts = 0;
    let req = ConfigValidationRequest {
        config: Arc::new(fields),
    };
    assert!(
        req.config.validate(ValidationRequest).is_err(),
        "max_attempts == 0 must fail validation through the carried config"
    );
}

/// @covers: ConfigValidationRequest
#[test]
fn test_config_validation_request_independent_instances_validate_independently_edge() {
    let ok_req = ConfigValidationRequest { config: sample() };
    let mut bad_fields = valid_fields();
    bad_fields.rate_limit_max_backoff_ms = 0;
    bad_fields.rate_limit_initial_backoff_ms = 1;
    let err_req = ConfigValidationRequest {
        config: Arc::new(bad_fields),
    };
    // Two independently-constructed requests must not interfere with each
    // other — one valid, one invalid, each judged solely on its own config.
    assert!(ok_req.config.validate(ValidationRequest).is_ok());
    assert!(err_req.config.validate(ValidationRequest).is_err());
}
