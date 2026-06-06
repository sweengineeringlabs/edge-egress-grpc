//! Core layer — breaker state-transition logic + decorator impl.

pub(crate) mod breaker_egress;
pub(crate) mod breaker_transition;
pub(crate) mod default_processor;
pub(crate) mod failure_classifier;
