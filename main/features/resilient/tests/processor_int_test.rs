//! Integration tests for the `Processor` trait contract on `GrpcResilientSvc`.

use swe_edge_egress_grpc_resilient::{GrpcResilientSvc, Processor};

/// @covers: Processor — trait is object-safe
#[test]
fn grpc_resilient_processor_is_object_safe_int_test() {
    fn _assert(_: &dyn Processor) {}
}

/// @covers: Processor::describe — returns expected label
#[test]
fn grpc_resilient_svc_describe_returns_grpc_resilient_label_int_test() {
    let svc = GrpcResilientSvc;
    assert_eq!(svc.describe(), "grpc-resilient");
}
