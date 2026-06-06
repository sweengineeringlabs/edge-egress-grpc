//! Core layer — decorator impl + backoff schedule logic + processor.

pub(crate) mod backoff_scheduler;
pub(crate) mod default_processor;
pub(crate) mod jitter_rng;
pub(crate) mod retry_egress;
