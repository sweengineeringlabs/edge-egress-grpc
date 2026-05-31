//! Core state-transition logic for the circuit breaker.

mod breaker_transition;

pub(crate) use breaker_transition::BreakerTransition;
