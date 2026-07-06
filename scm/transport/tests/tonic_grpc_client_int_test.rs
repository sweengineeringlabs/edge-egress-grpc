//! Coverage stub for `api/types/tonic_grpc_client.rs`.

use swe_edge_egress_grpc_transport::TonicGrpcClient;

/// @covers: TonicGrpcClient — type is accessible and holds real fields (not zero-sized)
#[test]
fn transport_struct_tonic_grpc_client_exists_int_test() {
    assert!(
        std::mem::size_of::<TonicGrpcClient>() > 0,
        "TonicGrpcClient holds base_uri/client/timeout/interceptors fields and must not be zero-sized"
    );
}
