//! Integration tests for `ValidationRequest`.
#![allow(clippy::default_constructed_unit_structs)] // the point of these tests is verifying ::default() matches the literal

use swe_edge_egress_grpc_transport::ValidationRequest;

/// @covers: ValidationRequest
#[test]
fn test_validation_request_default_constructs_happy() {
    let req = ValidationRequest;
    assert_eq!(format!("{req:?}"), "ValidationRequest");
}

/// @covers: ValidationRequest
#[test]
fn test_validation_request_default_trait_matches_literal_error() {
    assert_eq!(
        format!("{:?}", ValidationRequest::default()),
        "ValidationRequest"
    );
}

/// @covers: ValidationRequest
#[test]
fn test_validation_request_copy_is_independent_edge() {
    let a = ValidationRequest;
    let b = a;
    assert_eq!(format!("{a:?}"), "ValidationRequest");
    assert_eq!(format!("{b:?}"), "ValidationRequest");
    assert_eq!(std::mem::size_of_val(&a), 0);
}
