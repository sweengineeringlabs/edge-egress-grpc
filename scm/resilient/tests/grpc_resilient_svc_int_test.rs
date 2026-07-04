//! Rule 120 coverage stub for `src/api/types/grpc_resilient_svc.rs`.

/// @covers: GrpcResilientSvc struct is publicly accessible and is a zero-sized namespace marker
#[test]
fn resilient_struct_grpc_resilient_svc_is_accessible_int_test() {
    assert_eq!(
        std::mem::size_of::<swe_edge_egress_grpc_resilient::GrpcResilientSvc>(),
        0,
        "GrpcResilientSvc is a namespace marker for factory fns and must carry no instance state"
    );
}
