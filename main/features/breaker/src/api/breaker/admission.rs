//! Admission decision for incoming requests at the circuit breaker.

/// Decision returned when a new request arrives at the breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    /// Pass through — record the outcome afterward.
    Proceed,
    /// Breaker is open — fail fast without calling the inner client.
    RejectOpen,
}
