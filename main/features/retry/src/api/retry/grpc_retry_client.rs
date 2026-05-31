//! Interface counterpart for core::retry::grpc_retry_client.

/// Marker trait for gRPC retry client decorator implementations.
pub trait GrpcRetryClient: Send + Sync {}
