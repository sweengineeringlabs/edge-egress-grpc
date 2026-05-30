//! Core state-transition logic for the circuit breaker.

mod r#impl;

pub(crate) use r#impl::{admit, record};
