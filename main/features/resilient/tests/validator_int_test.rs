//! Integration tests for the `Validator` trait contract.

use swe_edge_egress_grpc_resilient::Validator;

/// @covers: Validator — trait is object-safe
#[test]
fn resilient_validator_is_object_safe_int_test() {
    fn _assert(_: &dyn Validator) {}
}
