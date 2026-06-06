#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `ResilienceConfigBuilder`.

use swe_edge_egress_grpc_transport::ResilienceConfigBuilder;

fn full_builder() -> ResilienceConfigBuilder {
    ResilienceConfigBuilder::new()
        .max_attempts(3)
        .rate_limit_max_attempts(2)
}

/// @covers: ResilienceConfigBuilder::build — valid config returns Ok
#[test]
fn transport_struct_resilience_config_builder_build_valid_config_returns_ok_int_test() {
    assert!(full_builder().build().is_ok());
}

/// @covers: ResilienceConfigBuilder::max_attempts — sets field
#[test]
fn transport_struct_resilience_config_builder_max_attempts_sets_field_int_test() {
    let c = full_builder().max_attempts(5).build().expect("build");
    assert_eq!(c.max_attempts, 5);
}

/// @covers: ResilienceConfigBuilder::initial_backoff_ms — sets field
#[test]
fn transport_struct_resilience_config_builder_initial_backoff_ms_sets_field_int_test() {
    let c = full_builder()
        .initial_backoff_ms(200)
        .build()
        .expect("build");
    assert_eq!(c.initial_backoff_ms, 200);
}

/// @covers: ResilienceConfigBuilder::backoff_multiplier — sets field
#[test]
fn transport_struct_resilience_config_builder_backoff_multiplier_sets_field_int_test() {
    let c = full_builder()
        .backoff_multiplier(3.0)
        .build()
        .expect("build");
    assert!((c.backoff_multiplier - 3.0).abs() < f64::EPSILON);
}

/// @covers: ResilienceConfigBuilder::jitter_factor — sets field
#[test]
fn transport_struct_resilience_config_builder_jitter_factor_sets_field_int_test() {
    let c = full_builder().jitter_factor(0.2).build().expect("build");
    assert!((c.jitter_factor - 0.2).abs() < f64::EPSILON);
}

/// @covers: ResilienceConfigBuilder::max_backoff_ms — sets field
#[test]
fn transport_struct_resilience_config_builder_max_backoff_ms_sets_field_int_test() {
    let c = full_builder().max_backoff_ms(9000).build().expect("build");
    assert_eq!(c.max_backoff_ms, 9000);
}

/// @covers: ResilienceConfigBuilder::rate_limit_max_attempts — sets field
#[test]
fn transport_struct_resilience_config_builder_rate_limit_max_attempts_sets_field_int_test() {
    let c = full_builder()
        .rate_limit_max_attempts(4)
        .build()
        .expect("build");
    assert_eq!(c.rate_limit_max_attempts, 4);
}

/// @covers: ResilienceConfigBuilder::rate_limit_initial_backoff_ms — sets field
#[test]
fn transport_struct_resilience_config_builder_rate_limit_initial_backoff_ms_sets_field_int_test() {
    let c = full_builder()
        .rate_limit_initial_backoff_ms(500)
        .build()
        .expect("build");
    assert_eq!(c.rate_limit_initial_backoff_ms, 500);
}

/// @covers: ResilienceConfigBuilder::rate_limit_max_backoff_ms — sets field
#[test]
fn transport_struct_resilience_config_builder_rate_limit_max_backoff_ms_sets_field_int_test() {
    let c = full_builder()
        .rate_limit_max_backoff_ms(20_000)
        .build()
        .expect("build");
    assert_eq!(c.rate_limit_max_backoff_ms, 20_000);
}

/// @covers: ResilienceConfigBuilder::failure_threshold — sets field
#[test]
fn transport_struct_resilience_config_builder_failure_threshold_sets_field_int_test() {
    let c = full_builder().failure_threshold(10).build().expect("build");
    assert_eq!(c.failure_threshold, 10);
}

/// @covers: ResilienceConfigBuilder::cool_down_seconds — sets field
#[test]
fn transport_struct_resilience_config_builder_cool_down_seconds_sets_field_int_test() {
    let c = full_builder().cool_down_seconds(60).build().expect("build");
    assert_eq!(c.cool_down_seconds, 60);
}

/// @covers: ResilienceConfigBuilder::half_open_probe_count — sets field
#[test]
fn transport_struct_resilience_config_builder_half_open_probe_count_sets_field_int_test() {
    let c = full_builder()
        .half_open_probe_count(3)
        .build()
        .expect("build");
    assert_eq!(c.half_open_probe_count, 3);
}

/// @covers: ResilienceConfigBuilder::build — missing required field returns Err
#[test]
fn transport_struct_resilience_config_builder_missing_required_field_returns_err_int_test() {
    let r = ResilienceConfigBuilder::new()
        .rate_limit_max_attempts(2)
        .build();
    assert!(r.is_err());
}
