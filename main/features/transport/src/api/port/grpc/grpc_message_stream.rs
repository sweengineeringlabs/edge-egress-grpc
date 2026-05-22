//! `GrpcMessageStream` — stream of gRPC message payloads.

use crate::api::port::grpc::grpc_egress_result::GrpcEgressResult;

/// A stream of gRPC message payloads (each item is a raw decoded frame body).
pub type GrpcMessageStream =
    std::pin::Pin<Box<dyn futures::Stream<Item = GrpcEgressResult<Vec<u8>>> + Send>>;

#[cfg(test)]
mod tests {
    #[test]
    fn test_grpc_message_stream_can_be_constructed_from_empty_stream() {
        use super::GrpcMessageStream;
        use crate::api::port::grpc::grpc_egress_result::GrpcEgressResult;
        use futures::stream;

        let s: GrpcMessageStream = Box::pin(stream::empty::<GrpcEgressResult<Vec<u8>>>());
        let _ = s;
    }
}
