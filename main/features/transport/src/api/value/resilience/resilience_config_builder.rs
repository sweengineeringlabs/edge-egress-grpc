//! `ResilienceConfigBuilder` — builder for [`ResilienceConfig`].

use super::resilience_config::ResilienceConfig;

/// Builder for [`ResilienceConfig`].
#[derive(Debug, Default)]
pub struct ResilienceConfigBuilder {
    max_attempts: Option<u32>,
    initial_backoff_ms: Option<u64>,
    backoff_multiplier: Option<f64>,
    jitter_factor: Option<f64>,
    max_backoff_ms: Option<u64>,
    rate_limit_max_attempts: Option<u32>,
    rate_limit_initial_backoff_ms: Option<u64>,
    rate_limit_max_backoff_ms: Option<u64>,
    failure_threshold: Option<u32>,
    cool_down_seconds: Option<u64>,
    half_open_probe_count: Option<u32>,
}

impl ResilienceConfigBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the maximum number of retry attempts.
    pub fn max_attempts(mut self, v: u32) -> Self {
        self.max_attempts = Some(v);
        self
    }
    /// Set the initial backoff delay in milliseconds.
    pub fn initial_backoff_ms(mut self, v: u64) -> Self {
        self.initial_backoff_ms = Some(v);
        self
    }
    /// Set the exponential backoff multiplier.
    pub fn backoff_multiplier(mut self, v: f64) -> Self {
        self.backoff_multiplier = Some(v);
        self
    }
    /// Set the jitter factor applied to each backoff interval (0.0–1.0).
    pub fn jitter_factor(mut self, v: f64) -> Self {
        self.jitter_factor = Some(v);
        self
    }
    /// Set the maximum backoff delay cap in milliseconds.
    pub fn max_backoff_ms(mut self, v: u64) -> Self {
        self.max_backoff_ms = Some(v);
        self
    }
    /// Set the maximum retry attempts for rate-limited responses.
    pub fn rate_limit_max_attempts(mut self, v: u32) -> Self {
        self.rate_limit_max_attempts = Some(v);
        self
    }
    /// Set the initial backoff delay for rate-limited retries in milliseconds.
    pub fn rate_limit_initial_backoff_ms(mut self, v: u64) -> Self {
        self.rate_limit_initial_backoff_ms = Some(v);
        self
    }
    /// Set the maximum backoff delay cap for rate-limited retries in milliseconds.
    pub fn rate_limit_max_backoff_ms(mut self, v: u64) -> Self {
        self.rate_limit_max_backoff_ms = Some(v);
        self
    }
    /// Set the circuit-breaker failure threshold before tripping.
    pub fn failure_threshold(mut self, v: u32) -> Self {
        self.failure_threshold = Some(v);
        self
    }
    /// Set the circuit-breaker cool-down period in seconds.
    pub fn cool_down_seconds(mut self, v: u64) -> Self {
        self.cool_down_seconds = Some(v);
        self
    }
    /// Set the number of probe requests to allow during half-open state.
    pub fn half_open_probe_count(mut self, v: u32) -> Self {
        self.half_open_probe_count = Some(v);
        self
    }

    /// Build the [`ResilienceConfig`]. Returns `Err` when any required field is unset.
    pub fn build(self) -> Result<ResilienceConfig, String> {
        Ok(ResilienceConfig {
            max_attempts: self.max_attempts.ok_or("max_attempts required")?,
            initial_backoff_ms: self.initial_backoff_ms.unwrap_or(100),
            backoff_multiplier: self.backoff_multiplier.unwrap_or(2.0),
            jitter_factor: self.jitter_factor.unwrap_or(0.1),
            max_backoff_ms: self.max_backoff_ms.unwrap_or(5_000),
            rate_limit_max_attempts: self
                .rate_limit_max_attempts
                .ok_or("rate_limit_max_attempts required")?,
            rate_limit_initial_backoff_ms: self.rate_limit_initial_backoff_ms.unwrap_or(1_000),
            rate_limit_max_backoff_ms: self.rate_limit_max_backoff_ms.unwrap_or(30_000),
            failure_threshold: self.failure_threshold.unwrap_or(5),
            cool_down_seconds: self.cool_down_seconds.unwrap_or(10),
            half_open_probe_count: self.half_open_probe_count.unwrap_or(1),
        })
    }
}
