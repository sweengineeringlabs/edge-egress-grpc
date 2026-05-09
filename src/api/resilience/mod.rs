//! Resilience interface layer — api/ counterpart for `core/resilience/`.

pub mod circuit_breaker;
pub mod resilience_validator;
pub mod resilient_grpc_client;
pub mod retry;

pub use circuit_breaker::CircuitBreaker;
pub use retry::{RetryOutcome, RetryPolicyPort};
