//! Outcome of a dispatched request as seen by the circuit breaker.

/// Outcome of a dispatched request, as seen by the breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Call returned `Ok` (or a non-breaker-failure error).
    Success,
    /// Call returned a result classified as a breaker failure
    /// (transport-level Unavailable, status Unavailable, or
    /// status/transport Internal).  See `api::failure_kind`.
    Failure,
}
