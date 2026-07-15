//! Integration tests for `ValidatorFactory`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_transport_grpc_egress_transport::{ValidationRequest, ValidatorFactory};

/// @covers: create
#[test]
fn test_create_returns_validator_accepting_default_config_happy() {
    let validator = ValidatorFactory::create();
    validator
        .validate(ValidationRequest)
        .expect("the default resilience config must validate cleanly");
}

/// @covers: create
#[test]
fn test_create_each_call_returns_independent_instance_edge() {
    let a = ValidatorFactory::create();
    let b = ValidatorFactory::create();
    // Each call must allocate its own boxed instance, not share one — proven
    // by distinct heap addresses, not just "both happen to validate ok".
    assert!(
        !std::ptr::eq(a.as_ref(), b.as_ref()),
        "two calls to create() must return independently-allocated instances"
    );
}

/// @covers: create
#[test]
fn test_create_validate_is_idempotent_across_repeated_calls_edge() {
    let validator = ValidatorFactory::create();
    let first = validator.validate(ValidationRequest).is_ok();
    let second = validator.validate(ValidationRequest).is_ok();
    assert_eq!(
        first, second,
        "repeated validate() calls on the same instance must agree"
    );
}
