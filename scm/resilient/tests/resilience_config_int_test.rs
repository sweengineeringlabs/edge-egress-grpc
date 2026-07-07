//! Integration tests for [`ResilienceConfig`] (local newtype wrapping the
//! transport crate's foreign `ResilienceConfig`).

use swe_edge_egress_grpc::ResilienceConfigResilienceValidator as ForeignResilienceConfig;
use swe_edge_egress_grpc_resilient::ResilienceConfig;

/// @covers: ResilienceConfig
#[test]
fn test_resilience_config_wraps_foreign_type_happy() {
    let wrapped = ResilienceConfig(ForeignResilienceConfig::default());
    assert_eq!(
        wrapped.0.max_attempts,
        ForeignResilienceConfig::default().max_attempts
    );
}

/// @covers: ResilienceConfig
#[test]
fn test_resilience_config_zero_max_attempts_is_preserved_error() {
    let cfg = ForeignResilienceConfig {
        max_attempts: 0,
        ..ForeignResilienceConfig::default()
    };
    let wrapped = ResilienceConfig(cfg);
    // The newtype itself does not validate — it's a pure wrapper; proving
    // it faithfully preserves an invalid value (not silently correcting
    // it) confirms it's a real wrapper, not a stub.
    assert_eq!(wrapped.0.max_attempts, 0);
}

/// @covers: ResilienceConfig
#[test]
fn test_resilience_config_distinct_instances_are_independent_edge() {
    let a = ResilienceConfig(ForeignResilienceConfig::default());
    let cfg = ForeignResilienceConfig {
        failure_threshold: 99,
        ..ForeignResilienceConfig::default()
    };
    let b = ResilienceConfig(cfg);
    assert_ne!(a.0.failure_threshold, b.0.failure_threshold);
}
