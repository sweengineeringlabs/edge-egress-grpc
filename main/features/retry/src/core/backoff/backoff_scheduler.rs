//! `BackoffScheduler` — stateless backoff computation helper.

use std::time::Duration;

use crate::api::types::grpc_retry_config::GrpcRetryConfig;

use super::jitter_rng::{next_backoff, rate_limit_backoff};

/// Stateless backoff computation helper.
///
/// Wraps the free-standing backoff schedule functions as associated methods
/// to satisfy SEA rule 191 (all functions must be methods on a type).
pub(crate) struct BackoffScheduler;

impl BackoffScheduler {
    /// Compute next standard-retry backoff.
    pub(crate) fn next_backoff(
        config: &GrpcRetryConfig,
        attempt: u32,
        random_unit: f64,
    ) -> Duration {
        next_backoff(config, attempt, random_unit)
    }

    /// Compute rate-limit backoff.
    pub(crate) fn rate_limit_backoff(
        config: &GrpcRetryConfig,
        attempt: u32,
        retry_after_hint: Option<Duration>,
        random_unit: f64,
    ) -> Duration {
        rate_limit_backoff(config, attempt, retry_after_hint, random_unit)
    }
}
