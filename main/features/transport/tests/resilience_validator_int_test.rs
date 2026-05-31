//! Integration tests for `ResilienceValidator` trait.

use swe_edge_egress_grpc_transport::ResilienceValidator;

/// @covers: ResilienceValidator is object-safe
#[test]
fn transport_trait_resilience_validator_is_object_safe_int_test() {
    fn _assert(_: &dyn ResilienceValidator) {}
}
