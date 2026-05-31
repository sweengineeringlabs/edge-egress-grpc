//! gRPC response envelope.

use super::grpc_metadata::GrpcMetadata;

/// A gRPC response envelope.
#[derive(Debug, Clone)]
pub struct GrpcResponse {
    pub body: Vec<u8>,
    pub metadata: GrpcMetadata,
}
