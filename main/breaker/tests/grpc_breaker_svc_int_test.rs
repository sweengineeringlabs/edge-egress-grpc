//! Rule 120 coverage stub for `src/api/types/grpc/grpc_breaker_svc.rs`.

/// @covers: GrpcBreakerSvc struct is publicly accessible
#[test]
fn breaker_struct_grpc_breaker_svc_is_accessible_int_test() {
    let _ = std::mem::size_of::<swe_edge_egress_grpc_breaker::GrpcBreakerSvc>();
}
