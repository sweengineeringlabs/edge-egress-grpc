//! Interface counterpart for core::backoff::backoff_scheduler.

/// Marker trait for backoff schedule computation.
#[expect(
    dead_code,
    reason = "SEA api/ counterpart — structural anchor, not yet used"
)]
pub trait BackoffScheduler: Send + Sync {}
