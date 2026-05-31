//! Integration tests for `api/value/grpc/grpc_response.rs`.

use swe_edge_egress_grpc_transport::{GrpcMetadata, GrpcResponse};

#[test]
fn transport_struct_grpc_response_holds_body_bytes_int_test() {
    let resp = GrpcResponse {
        body: vec![0x08, 0x01],
        metadata: GrpcMetadata::default(),
    };
    assert_eq!(resp.body, vec![0x08, 0x01]);
}
