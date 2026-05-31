//! Backoff schedule interface — types for exponential-backoff computation.
pub use crate::api::types::BackoffSchedule;
pub(crate) mod backoff_scheduler;
pub(crate) mod jitter_rng;
