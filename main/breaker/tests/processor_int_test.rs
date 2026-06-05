//! Integration tests for the `Processor` trait contract on `GrpcBreakerSvc`.

use swe_edge_egress_grpc_breaker::{GrpcBreakerSvc, Processor};

/// @covers: Processor — trait is object-safe
#[test]
fn breaker_trait_processor_is_object_safe_int_test() {
    fn _assert(_: &dyn Processor) {}
}

/// @covers: Processor::describe — returns expected label
#[test]
fn breaker_struct_grpc_breaker_svc_describe_returns_label_int_test() {
    let svc = GrpcBreakerSvc;
    assert_eq!(svc.describe(), "grpc-breaker");
}
