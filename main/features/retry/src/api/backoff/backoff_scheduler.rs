//! Interface counterpart for core::backoff::backoff_scheduler.

/// Marker trait for backoff schedule computation.
pub trait BackoffScheduler: Send + Sync {}
