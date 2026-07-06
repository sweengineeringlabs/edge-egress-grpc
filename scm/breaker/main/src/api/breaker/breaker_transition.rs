//! Shared constant for the state-transition implementation — the flat api/
//! counterpart to the flat `core::breaker::breaker_transition` file.

/// Label used in `tracing` events emitted during state transitions.
pub const BREAKER_TRANSITION_LOG_TARGET: &str = "grpc_breaker::transition";
