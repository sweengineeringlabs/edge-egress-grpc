//! `ApplicationConfigBuilder` — typed builder for `config/application.toml`.
//!
//! Mirrors the TOML structure of `config/application.toml` (resilience block)
//! and provides a fluent API for constructing and overriding configuration
//! at runtime — typically in tests or when assembling a transport from
//! non-file-backed config sources.

use crate::api::value_object::ResilienceConfig;

/// Typed builder for `config/application.toml`.
///
/// Use [`Self::default`] to start with the built-in defaults (fast stateless
/// gRPC profile), then override individual fields and call [`Self::build`].
///
/// # Example
///
/// ```ignore
/// let config = ApplicationConfigBuilder::default()
///     .max_attempts(3)
///     .cool_down_seconds(30)
///     .build();
/// ```
#[allow(dead_code)]
pub struct ApplicationConfigBuilder {
    max_attempts: u32,
    initial_backoff_ms: u64,
    backoff_multiplier: f64,
    jitter_factor: f64,
    max_backoff_ms: u64,
    rate_limit_max_attempts: u32,
    rate_limit_initial_backoff_ms: u64,
    rate_limit_max_backoff_ms: u64,
    failure_threshold: u32,
    cool_down_seconds: u64,
    half_open_probe_count: u32,
}

impl Default for ApplicationConfigBuilder {
    /// Returns a builder pre-populated with the `config/application.toml` defaults.
    fn default() -> Self {
        let d = ResilienceConfig::default();
        Self {
            max_attempts: d.max_attempts,
            initial_backoff_ms: d.initial_backoff_ms,
            backoff_multiplier: d.backoff_multiplier,
            jitter_factor: d.jitter_factor,
            max_backoff_ms: d.max_backoff_ms,
            rate_limit_max_attempts: d.rate_limit_max_attempts,
            rate_limit_initial_backoff_ms: d.rate_limit_initial_backoff_ms,
            rate_limit_max_backoff_ms: d.rate_limit_max_backoff_ms,
            failure_threshold: d.failure_threshold,
            cool_down_seconds: d.cool_down_seconds,
            half_open_probe_count: d.half_open_probe_count,
        }
    }
}

#[allow(dead_code)]
impl ApplicationConfigBuilder {
    /// Override the maximum retry attempt count (including the initial call).
    pub fn max_attempts(mut self, v: u32) -> Self {
        self.max_attempts = v;
        self
    }

    /// Override the initial standard-retry backoff in milliseconds.
    pub fn initial_backoff_ms(mut self, v: u64) -> Self {
        self.initial_backoff_ms = v;
        self
    }

    /// Override the exponential backoff growth factor.
    pub fn backoff_multiplier(mut self, v: f64) -> Self {
        self.backoff_multiplier = v;
        self
    }

    /// Override the jitter fraction (0.0 = none, 0.1 = ±10%).
    pub fn jitter_factor(mut self, v: f64) -> Self {
        self.jitter_factor = v;
        self
    }

    /// Override the hard cap on standard retry backoff in milliseconds.
    pub fn max_backoff_ms(mut self, v: u64) -> Self {
        self.max_backoff_ms = v;
        self
    }

    /// Override the rate-limit retry attempt ceiling.
    pub fn rate_limit_max_attempts(mut self, v: u32) -> Self {
        self.rate_limit_max_attempts = v;
        self
    }

    /// Override the rate-limit initial backoff in milliseconds.
    pub fn rate_limit_initial_backoff_ms(mut self, v: u64) -> Self {
        self.rate_limit_initial_backoff_ms = v;
        self
    }

    /// Override the hard cap on rate-limit backoff in milliseconds.
    pub fn rate_limit_max_backoff_ms(mut self, v: u64) -> Self {
        self.rate_limit_max_backoff_ms = v;
        self
    }

    /// Override the consecutive-failure threshold that opens the circuit breaker.
    pub fn failure_threshold(mut self, v: u32) -> Self {
        self.failure_threshold = v;
        self
    }

    /// Override the cool-down window in seconds before the breaker probes again.
    pub fn cool_down_seconds(mut self, v: u64) -> Self {
        self.cool_down_seconds = v;
        self
    }

    /// Override the number of consecutive probe successes required to close the breaker.
    pub fn half_open_probe_count(mut self, v: u32) -> Self {
        self.half_open_probe_count = v;
        self
    }

    /// Consume the builder and return the configured [`ResilienceConfig`].
    pub fn build(self) -> ResilienceConfig {
        ResilienceConfig {
            max_attempts: self.max_attempts,
            initial_backoff_ms: self.initial_backoff_ms,
            backoff_multiplier: self.backoff_multiplier,
            jitter_factor: self.jitter_factor,
            max_backoff_ms: self.max_backoff_ms,
            rate_limit_max_attempts: self.rate_limit_max_attempts,
            rate_limit_initial_backoff_ms: self.rate_limit_initial_backoff_ms,
            rate_limit_max_backoff_ms: self.rate_limit_max_backoff_ms,
            failure_threshold: self.failure_threshold,
            cool_down_seconds: self.cool_down_seconds,
            half_open_probe_count: self.half_open_probe_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_matches_resilience_config_default() {
        let built = ApplicationConfigBuilder::default().build();
        let expected = ResilienceConfig::default();
        assert_eq!(built.max_attempts, expected.max_attempts);
        assert_eq!(built.failure_threshold, expected.failure_threshold);
        assert_eq!(built.cool_down_seconds, expected.cool_down_seconds);
    }

    /// @covers: max_attempts
    #[test]
    fn test_max_attempts_overrides_default() {
        let built = ApplicationConfigBuilder::default().max_attempts(2).build();
        assert_eq!(built.max_attempts, 2);
    }

    /// @covers: initial_backoff_ms
    #[test]
    fn test_initial_backoff_ms_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .initial_backoff_ms(500)
            .build();
        assert_eq!(built.initial_backoff_ms, 500);
    }

    /// @covers: backoff_multiplier
    #[test]
    fn test_backoff_multiplier_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .backoff_multiplier(3.0)
            .build();
        assert!((built.backoff_multiplier - 3.0).abs() < f64::EPSILON);
    }

    /// @covers: jitter_factor
    #[test]
    fn test_jitter_factor_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .jitter_factor(0.5)
            .build();
        assert!((built.jitter_factor - 0.5).abs() < f64::EPSILON);
    }

    /// @covers: max_backoff_ms
    #[test]
    fn test_max_backoff_ms_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .max_backoff_ms(10_000)
            .build();
        assert_eq!(built.max_backoff_ms, 10_000);
    }

    /// @covers: rate_limit_max_attempts
    #[test]
    fn test_rate_limit_max_attempts_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .rate_limit_max_attempts(1)
            .build();
        assert_eq!(built.rate_limit_max_attempts, 1);
    }

    /// @covers: rate_limit_initial_backoff_ms
    #[test]
    fn test_rate_limit_initial_backoff_ms_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .rate_limit_initial_backoff_ms(2_000)
            .build();
        assert_eq!(built.rate_limit_initial_backoff_ms, 2_000);
    }

    /// @covers: rate_limit_max_backoff_ms
    #[test]
    fn test_rate_limit_max_backoff_ms_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .rate_limit_max_backoff_ms(30_000)
            .build();
        assert_eq!(built.rate_limit_max_backoff_ms, 30_000);
    }

    /// @covers: failure_threshold
    #[test]
    fn test_failure_threshold_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .failure_threshold(3)
            .build();
        assert_eq!(built.failure_threshold, 3);
    }

    /// @covers: cool_down_seconds
    #[test]
    fn test_cool_down_seconds_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .cool_down_seconds(60)
            .build();
        assert_eq!(built.cool_down_seconds, 60);
    }

    /// @covers: half_open_probe_count
    #[test]
    fn test_half_open_probe_count_overrides_default() {
        let built = ApplicationConfigBuilder::default()
            .half_open_probe_count(2)
            .build();
        assert_eq!(built.half_open_probe_count, 2);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_valid_resilience_config() {
        let cfg = ApplicationConfigBuilder::default()
            .max_attempts(3)
            .initial_backoff_ms(200)
            .backoff_multiplier(1.5)
            .jitter_factor(0.2)
            .max_backoff_ms(5_000)
            .rate_limit_max_attempts(1)
            .rate_limit_initial_backoff_ms(2_000)
            .rate_limit_max_backoff_ms(30_000)
            .failure_threshold(3)
            .cool_down_seconds(45)
            .half_open_probe_count(2)
            .build();

        assert_eq!(cfg.max_attempts, 3);
        assert_eq!(cfg.initial_backoff_ms, 200);
        assert!((cfg.backoff_multiplier - 1.5).abs() < f64::EPSILON);
        assert_eq!(cfg.failure_threshold, 3);
        assert_eq!(cfg.cool_down_seconds, 45);
        assert_eq!(cfg.half_open_probe_count, 2);
    }
}
