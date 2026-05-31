//! Integration tests for the `traits` module (`Processor`, `Validator`).

use swe_edge_egress_grpc_transport::{GrpcEgress, Processor, Validator};

/// @covers: GrpcEgress is object-safe
#[test]
fn transport_trait_grpc_egress_is_object_safe_int_test() {
    fn _assert(_: &dyn GrpcEgress) {}
}

/// @covers: Processor is object-safe
#[test]
fn transport_trait_processor_is_object_safe_int_test() {
    fn _assert(_: &dyn Processor) {}
}

/// @covers: Validator is object-safe
#[test]
fn transport_trait_validator_is_object_safe_int_test() {
    fn _assert(_: &dyn Validator) {}
}
