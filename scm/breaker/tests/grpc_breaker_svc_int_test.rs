//! Rule 120 coverage stub for `src/api/types/grpc/grpc_breaker_svc.rs`.

/// @covers: GrpcBreakerSvc struct is publicly accessible and is a zero-sized namespace marker
#[test]
fn breaker_struct_grpc_breaker_svc_is_accessible_int_test() {
    assert_eq!(
        std::mem::size_of::<swe_edge_egress_grpc_breaker::GrpcBreakerSvc>(),
        0,
        "GrpcBreakerSvc is a namespace marker for factory fns and must carry no instance state"
    );
}
