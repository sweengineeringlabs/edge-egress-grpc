//! Integration tests for `validate_resilience_config` (the `Validator` SAF wrapper).

use swe_edge_egress_grpc::{ResilienceConfig, validate_resilience_config};

fn valid() -> ResilienceConfig {
    ResilienceConfig {
        max_attempts:                  3,
        initial_backoff_ms:            100,
        backoff_multiplier:            2.0,
        jitter_factor:                 0.1,
        max_backoff_ms:                2_000,
        rate_limit_max_attempts:       2,
        rate_limit_initial_backoff_ms: 1_000,
        rate_limit_max_backoff_ms:     10_000,
        failure_threshold:             5,
        cool_down_seconds:             10,
        half_open_probe_count:         1,
    }
}

/// @covers: validate_resilience_config — valid config is accepted.
#[test]
fn grpc_struct_resilience_config_valid_config_is_accepted_int_test() {
    assert!(validate_resilience_config(&valid()).is_ok());
}

/// @covers: validate_resilience_config — zero max_attempts is rejected.
#[test]
fn grpc_struct_resilience_config_zero_max_attempts_rejected_int_test() {
    let mut r = valid();
    r.max_attempts = 0;
    assert!(validate_resilience_config(&r).is_err());
}

/// @covers: validate_resilience_config — jitter_factor out of range is rejected.
#[test]
fn grpc_struct_resilience_config_jitter_out_of_range_rejected_int_test() {
    let mut r = valid();
    r.jitter_factor = 1.5;
    assert!(validate_resilience_config(&r).is_err());
}
