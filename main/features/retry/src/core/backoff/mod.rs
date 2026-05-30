//! Backoff computation — schedule and RNG types.

pub(crate) mod backoff_scheduler;
pub(crate) mod jitter_rng;

pub(crate) use backoff_scheduler::BackoffScheduler;
pub(crate) use jitter_rng::JitterRng;
