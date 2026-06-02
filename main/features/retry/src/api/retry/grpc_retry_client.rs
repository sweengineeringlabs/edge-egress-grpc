//! Interface counterpart for core::retry::grpc_retry_client.

/// Marker trait for gRPC retry client decorator implementations.
#[expect(dead_code, reason = "SEA api/ counterpart — structural anchor, not yet used")]
pub trait GrpcRetryClient: Send + Sync {}
