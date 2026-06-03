//! Interface counterpart for the corresponding core/ implementation.

/// Trait for jitter RNG implementations used in backoff computation.
#[expect(
    dead_code,
    reason = "SEA api/ counterpart — structural anchor, not yet used"
)]
pub trait JitterRng: Send + Sync {
    /// Return the next uniform sample in `[0.0, 1.0)`.
    fn next_unit(&mut self) -> f64;
}
