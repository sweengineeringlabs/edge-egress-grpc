//! Integration tests for `GrpcMessageStream`.

use swe_edge_egress_grpc_transport::{GrpcEgressResult, GrpcMessageStream};

/// @covers: GrpcMessageStream — can be constructed from an empty stream
#[test]
fn transport_type_grpc_message_stream_can_be_constructed_from_empty_stream_int_test() {
    use futures::stream;
    let s: GrpcMessageStream = Box::pin(stream::empty::<GrpcEgressResult<Vec<u8>>>());
    let _ = s;
}
