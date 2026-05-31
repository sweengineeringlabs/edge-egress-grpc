//! Interface counterpart for the corresponding core/ implementation.

/// Marker trait for jitter RNG implementations.
pub trait JitterRng: Send + Sync {}
