//! Integration tests for `ResilientGrpcClientPort` trait.

use swe_edge_egress_grpc_transport::{GrpcEgress, ResilientGrpcClientPort};

/// @covers: ResilientGrpcClientPort is object-safe
#[test]
fn transport_trait_resilient_grpc_client_port_is_object_safe_int_test() {
    fn _assert(_: &dyn ResilientGrpcClientPort) {}
}

/// @covers: GrpcEgress re-export is object-safe
#[test]
fn transport_trait_grpc_egress_is_object_safe_via_resilient_port_int_test() {
    fn _assert(_: &dyn GrpcEgress) {}
}
