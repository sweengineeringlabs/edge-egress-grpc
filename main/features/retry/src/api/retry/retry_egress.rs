//! Interface counterpart for `core::retry::RetryEgress`.

/// Interface contract for the retry `GrpcEgress` implementation.
#[expect(dead_code, reason = "SEA api/ counterpart — structural anchor, not yet used")]
pub trait RetryEgress: Send + Sync {}
