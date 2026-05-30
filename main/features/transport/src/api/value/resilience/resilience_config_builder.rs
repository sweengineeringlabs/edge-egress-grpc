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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn max_attempts(mut self, v: u32) -> Self {
        self.max_attempts = Some(v);
        self
    }
    pub fn initial_backoff_ms(mut self, v: u64) -> Self {
        self.initial_backoff_ms = Some(v);
        self
    }
    pub fn backoff_multiplier(mut self, v: f64) -> Self {
        self.backoff_multiplier = Some(v);
        self
    }
    pub fn jitter_factor(mut self, v: f64) -> Self {
        self.jitter_factor = Some(v);
        self
    }
    pub fn max_backoff_ms(mut self, v: u64) -> Self {
        self.max_backoff_ms = Some(v);
        self
    }
    pub fn rate_limit_max_attempts(mut self, v: u32) -> Self {
        self.rate_limit_max_attempts = Some(v);
        self
    }
    pub fn rate_limit_initial_backoff_ms(mut self, v: u64) -> Self {
        self.rate_limit_initial_backoff_ms = Some(v);
        self
    }
    pub fn rate_limit_max_backoff_ms(mut self, v: u64) -> Self {
        self.rate_limit_max_backoff_ms = Some(v);
        self
    }
    pub fn failure_threshold(mut self, v: u32) -> Self {
        self.failure_threshold = Some(v);
        self
    }
    pub fn cool_down_seconds(mut self, v: u64) -> Self {
        self.cool_down_seconds = Some(v);
        self
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn full_builder() -> ResilienceConfigBuilder {
        ResilienceConfigBuilder::new()
            .max_attempts(3)
            .rate_limit_max_attempts(2)
    }

    /// @covers: build
    #[test]
    fn test_build_valid_config_returns_ok() {
        assert!(full_builder().build().is_ok());
    }

    /// @covers: max_attempts
    #[test]
    fn test_max_attempts_sets_field() {
        let c = full_builder().max_attempts(5).build().unwrap();
        assert_eq!(c.max_attempts, 5);
    }

    /// @covers: initial_backoff_ms
    #[test]
    fn test_initial_backoff_ms_sets_field() {
        let c = full_builder().initial_backoff_ms(200).build().unwrap();
        assert_eq!(c.initial_backoff_ms, 200);
    }

    /// @covers: backoff_multiplier
    #[test]
    fn test_backoff_multiplier_sets_field() {
        let c = full_builder().backoff_multiplier(3.0).build().unwrap();
        assert!((c.backoff_multiplier - 3.0).abs() < f64::EPSILON);
    }

    /// @covers: jitter_factor
    #[test]
    fn test_jitter_factor_sets_field() {
        let c = full_builder().jitter_factor(0.2).build().unwrap();
        assert!((c.jitter_factor - 0.2).abs() < f64::EPSILON);
    }

    /// @covers: max_backoff_ms
    #[test]
    fn test_max_backoff_ms_sets_field() {
        let c = full_builder().max_backoff_ms(9000).build().unwrap();
        assert_eq!(c.max_backoff_ms, 9000);
    }

    /// @covers: rate_limit_max_attempts
    #[test]
    fn test_rate_limit_max_attempts_sets_field() {
        let c = full_builder().rate_limit_max_attempts(4).build().unwrap();
        assert_eq!(c.rate_limit_max_attempts, 4);
    }

    /// @covers: rate_limit_initial_backoff_ms
    #[test]
    fn test_rate_limit_initial_backoff_ms_sets_field() {
        let c = full_builder()
            .rate_limit_initial_backoff_ms(500)
            .build()
            .unwrap();
        assert_eq!(c.rate_limit_initial_backoff_ms, 500);
    }

    /// @covers: rate_limit_max_backoff_ms
    #[test]
    fn test_rate_limit_max_backoff_ms_sets_field() {
        let c = full_builder()
            .rate_limit_max_backoff_ms(20_000)
            .build()
            .unwrap();
        assert_eq!(c.rate_limit_max_backoff_ms, 20_000);
    }

    /// @covers: failure_threshold
    #[test]
    fn test_failure_threshold_sets_field() {
        let c = full_builder().failure_threshold(10).build().unwrap();
        assert_eq!(c.failure_threshold, 10);
    }

    /// @covers: cool_down_seconds
    #[test]
    fn test_cool_down_seconds_sets_field() {
        let c = full_builder().cool_down_seconds(60).build().unwrap();
        assert_eq!(c.cool_down_seconds, 60);
    }

    /// @covers: half_open_probe_count
    #[test]
    fn test_half_open_probe_count_sets_field() {
        let c = full_builder().half_open_probe_count(3).build().unwrap();
        assert_eq!(c.half_open_probe_count, 3);
    }

    /// @covers: build
    #[test]
    fn test_build_missing_required_field_returns_err() {
        let r = ResilienceConfigBuilder::new()
            .rate_limit_max_attempts(2)
            .build();
        assert!(r.is_err());
    }
}
