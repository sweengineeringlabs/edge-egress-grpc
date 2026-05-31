//! Interface counterpart for `core::breaker::client::BreakerEgress`.

/// Interface contract for the circuit-breaker `GrpcEgress` implementation.
pub trait BreakerEgress: Send + Sync {}
