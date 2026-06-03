#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `ResilienceConfig`.

use swe_edge_egress_grpc_transport::ResilienceConfig;

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

/// @covers: ResilienceConfig::default — returns the fast-stateless-gRPC profile
#[test]
fn transport_struct_resilience_config_default_returns_fast_stateless_grpc_profile_int_test() {
    let d = ResilienceConfig::default();
    assert_eq!(d.max_attempts, 5);
    assert_eq!(d.failure_threshold, 5);
    assert_eq!(d.cool_down_seconds, 30);
    assert_eq!(d.half_open_probe_count, 1);
    assert_eq!(d.initial_backoff_ms, 100);
    assert_eq!(d.rate_limit_max_attempts, 2);
}

/// @covers: ResilienceConfig serde — round-trips through TOML
#[test]
fn transport_struct_resilience_config_round_trips_through_toml_int_test() {
    let original = sample();
    let encoded = toml::to_string(&original).expect("serialize");
    let restored: ResilienceConfig = toml::from_str(&encoded).expect("deserialize");
    assert_eq!(restored.max_attempts, original.max_attempts);
    assert_eq!(restored.failure_threshold, original.failure_threshold);
    assert_eq!(restored.cool_down_seconds, original.cool_down_seconds);
    assert_eq!(
        restored.half_open_probe_count,
        original.half_open_probe_count
    );
    assert_eq!(
        restored.rate_limit_max_attempts,
        original.rate_limit_max_attempts
    );
}

/// @covers: ResilienceConfig — all fields survive TOML round trip
#[test]
fn transport_struct_resilience_config_all_fields_survive_round_trip_int_test() {
    let s = sample();
    let t = toml::to_string(&s).expect("serialize");
    let r: ResilienceConfig = toml::from_str(&t).expect("deserialize");
    assert_eq!(r.initial_backoff_ms, s.initial_backoff_ms);
    assert!((r.backoff_multiplier - s.backoff_multiplier).abs() < f64::EPSILON);
    assert!((r.jitter_factor - s.jitter_factor).abs() < f64::EPSILON);
    assert_eq!(r.max_backoff_ms, s.max_backoff_ms);
    assert_eq!(
        r.rate_limit_initial_backoff_ms,
        s.rate_limit_initial_backoff_ms
    );
    assert_eq!(r.rate_limit_max_backoff_ms, s.rate_limit_max_backoff_ms);
}
