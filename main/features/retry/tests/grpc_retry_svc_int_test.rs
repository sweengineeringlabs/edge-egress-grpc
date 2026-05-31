//! Rule 120 coverage stub for `src/api/types/grpc/grpc_retry_svc.rs`.

/// @covers: GrpcRetrySvc struct is publicly accessible
#[test]
fn retry_struct_grpc_retry_svc_is_accessible_int_test() {
    let _ = std::mem::size_of::<swe_edge_egress_grpc_retry::GrpcRetrySvc>();
}
