//! `GrpcMessageStream` — stream of gRPC message payloads.

use crate::api::types::GrpcEgressResult;

/// A stream of gRPC message payloads (each item is a raw decoded frame body).
pub type GrpcMessageStream =
    std::pin::Pin<Box<dyn futures::Stream<Item = GrpcEgressResult<Vec<u8>>> + Send>>;
