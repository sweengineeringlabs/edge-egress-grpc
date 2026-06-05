//! Integration tests for `api/interceptor/grpc/grpc_egress_interceptor.rs`.

use swe_edge_egress_grpc_transport::GrpcEgressInterceptor;

#[test]
fn transport_trait_grpc_egress_interceptor_is_object_safe_int_test() {
    fn _assert(_: &dyn GrpcEgressInterceptor) {}
}
