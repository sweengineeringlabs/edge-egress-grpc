//! Interface counterpart for `core::retry::RetryEgress`.

/// Interface contract for the retry `GrpcEgress` implementation.
pub trait RetryEgress: Send + Sync {}
