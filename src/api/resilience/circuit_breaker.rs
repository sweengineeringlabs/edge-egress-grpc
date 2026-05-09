//! `CircuitBreaker` trait — interface contract for circuit breaker implementations.

/// Interface contract for a circuit breaker.
///
/// Implemented by `crate::core::resilience::circuit_breaker::CircuitBreaker`.
pub trait CircuitBreaker: Send + Sync {
    /// Returns `true` when the circuit is open and calls should be rejected.
    fn is_open(&self) -> bool;
    /// Record a successful call — may transition Open → HalfOpen → Closed.
    fn record_success(&self);
    /// Record a failed call — may transition Closed → Open.
    fn record_failure(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_trait_is_object_safe() {
        fn _assert(_: &dyn CircuitBreaker) {}
    }
}
