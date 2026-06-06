//! Interface counterpart for `core::breaker_egress::BreakerEgress`.

/// Interface contract for the circuit-breaker `GrpcEgress` implementation.
#[expect(
    dead_code,
    reason = "SEA api/ counterpart — structural anchor, not yet used"
)]
pub trait BreakerEgress: Send + Sync {}
