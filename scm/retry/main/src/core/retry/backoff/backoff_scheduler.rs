//! `BackoffScheduler` — stateless backoff computation helper.

use std::time::Duration;

use crate::api::GrpcRetryConfig;

/// Stateless backoff computation helper.
pub(crate) struct BackoffScheduler;

impl BackoffScheduler {
    /// Compute next standard-retry backoff for the given attempt index (0-based).
    ///
    /// `random_unit` is a uniform value in `[0.0, 1.0)`.
    pub(crate) fn next_backoff(
        config: &GrpcRetryConfig,
        attempt: u32,
        random_unit: f64,
    ) -> Duration {
        debug_assert!((0.0..1.0).contains(&random_unit));
        Self::exponential_jitter(
            config.initial_backoff_ms,
            config.max_backoff_ms,
            config.backoff_multiplier,
            config.jitter_factor,
            attempt,
            random_unit,
        )
    }

    /// Compute rate-limit backoff.
    ///
    /// When `retry_after_hint` is `Some`, that value is returned directly.
    pub(crate) fn rate_limit_backoff(
        config: &GrpcRetryConfig,
        attempt: u32,
        retry_after_hint: Option<Duration>,
        random_unit: f64,
    ) -> Duration {
        if let Some(hint) = retry_after_hint {
            return hint;
        }
        debug_assert!((0.0..1.0).contains(&random_unit));
        Self::exponential_jitter(
            config.rate_limit_initial_backoff_ms,
            config.rate_limit_max_backoff_ms,
            config.backoff_multiplier,
            config.jitter_factor,
            attempt,
            random_unit,
        )
    }

    fn exponential_jitter(
        initial_ms: u64,
        max_ms: u64,
        multiplier: f64,
        jitter_factor: f64,
        attempt: u32,
        random_unit: f64,
    ) -> Duration {
        let base_ms = (initial_ms as f64) * multiplier.powi(attempt as i32);
        let capped_ms = base_ms.min(max_ms as f64);
        let jitter_mult = 1.0 - jitter_factor + (2.0 * jitter_factor * random_unit);
        let jittered_ms = capped_ms * jitter_mult;
        let final_ms = jittered_ms.min(max_ms as f64).max(0.0);
        Duration::from_millis(final_ms.round() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::GrpcRetryConfig;

    fn default_config() -> GrpcRetryConfig {
        GrpcRetryConfig::default()
    }

    #[test]
    fn test_next_backoff_attempt_zero_returns_initial_ms() {
        let cfg = default_config();
        let d = BackoffScheduler::next_backoff(&cfg, 0, 0.5);
        assert!(d.as_millis() > 0);
    }

    #[test]
    fn test_rate_limit_backoff_hint_overrides_exponential() {
        let cfg = default_config();
        let hint = Duration::from_secs(10);
        let d = BackoffScheduler::rate_limit_backoff(&cfg, 0, Some(hint), 0.5);
        assert_eq!(d, hint);
    }

    #[test]
    fn test_exponential_jitter_caps_at_max_ms() {
        // Attempt high enough that the exponential term would blow past
        // max_ms without the min() cap -- proves the cap is real, not
        // just coincidentally unreached by the other tests' small attempts.
        let d = BackoffScheduler::exponential_jitter(100, 500, 2.0, 0.0, 10, 0.5);
        assert_eq!(d, Duration::from_millis(500));
    }

    #[test]
    fn test_exponential_jitter_zero_jitter_factor_is_deterministic() {
        let a = BackoffScheduler::exponential_jitter(100, 10_000, 2.0, 0.0, 2, 0.1);
        let b = BackoffScheduler::exponential_jitter(100, 10_000, 2.0, 0.0, 2, 0.9);
        // jitter_factor 0.0 means random_unit has no effect on the result.
        assert_eq!(a, b);
        assert_eq!(a, Duration::from_millis(400));
    }
}
