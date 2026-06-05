//! Integration tests for `api/client/grpc_client_builder.rs`.

use swe_edge_egress_grpc_transport::GrpcClientBuilder;

#[test]
fn transport_struct_grpc_client_builder_is_constructable_int_test() {
    let _ = GrpcClientBuilder;
}
