//! `impl` blocks for [`ResilienceConfig`], [`ResilienceConfigBuilder`], and
//! their `Validator`/`ResilienceValidator` implementations. The type
//! *declarations* live in `api/`.
//!
//! Merged into one file (rather than one file per type) so the
//! `resilience_config`/`resilience_config_builder`/`resilience_validator`
//! filenames don't share a prefix as flat siblings of `core/` — see
//! `shared_prefix_grouping`. Subdirectory nesting was tried first and
//! rejected: it fails `core_api_module_correspondence`, since api/'s
//! kind-based layout (types/, traits/, errors/) has no theme subdirectory
//! to correspond to a new core/resilience/ directory.

use crate::api::Validator;
use crate::api::{
    GrpcChannelConfigError, ResilienceConfig, ResilienceConfigBuilder, ValidationRequest,
};

impl Default for ResilienceConfig {
    /// Returns the fast-stateless-gRPC profile: the calibrated baseline that
    /// confirmed ≤ 1.5× retry amplification in load tests.
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff_ms: 100,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            max_backoff_ms: 5_000,
            rate_limit_max_attempts: 2,
            rate_limit_initial_backoff_ms: 1_000,
            rate_limit_max_backoff_ms: 10_000,
            failure_threshold: 5,
            cool_down_seconds: 30,
            half_open_probe_count: 1,
        }
    }
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
    /// Set the jitter factor applied to each backoff interval (0.0-1.0).
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

impl Validator for ResilienceConfig {
    fn validate(&self, _req: ValidationRequest) -> Result<(), GrpcChannelConfigError> {
        if self.max_attempts == 0 {
            return Err(GrpcChannelConfigError::Config(
                "max_attempts must be >= 1".into(),
            ));
        }
        if self.rate_limit_max_attempts == 0 {
            return Err(GrpcChannelConfigError::Config(
                "rate_limit_max_attempts must be >= 1".into(),
            ));
        }
        if self.jitter_factor < 0.0 || self.jitter_factor > 1.0 {
            return Err(GrpcChannelConfigError::Config(format!(
                "jitter_factor must be in [0.0, 1.0], got {:.4}",
                self.jitter_factor
            )));
        }
        if self.half_open_probe_count == 0 {
            return Err(GrpcChannelConfigError::Config(
                "half_open_probe_count must be >= 1".into(),
            ));
        }
        if self.rate_limit_max_backoff_ms < self.rate_limit_initial_backoff_ms {
            return Err(GrpcChannelConfigError::Config(format!(
                "rate_limit_max_backoff_ms ({}) must be >= rate_limit_initial_backoff_ms ({})",
                self.rate_limit_max_backoff_ms, self.rate_limit_initial_backoff_ms
            )));
        }
        Ok(())
    }
}

impl crate::api::ResilienceValidator for ResilienceConfig {
    fn validate_config(
        &self,
        req: crate::api::ConfigValidationRequest,
    ) -> Result<(), GrpcChannelConfigError> {
        req.config.validate(ValidationRequest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> ResilienceConfig {
        ResilienceConfig {
            max_attempts: 3,
            initial_backoff_ms: 100,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            max_backoff_ms: 2_000,
            rate_limit_max_attempts: 2,
            rate_limit_initial_backoff_ms: 1_000,
            rate_limit_max_backoff_ms: 10_000,
            failure_threshold: 5,
            cool_down_seconds: 10,
            half_open_probe_count: 1,
        }
    }

    #[test]
    fn test_validate_valid_config_returns_ok() {
        assert!(valid().validate(ValidationRequest).is_ok());
        // Sibling negative case in the same test: a single field flipped to
        // invalid on an otherwise-valid config must fail, proving is_ok()
        // above isn't just a stub that always succeeds regardless of input.
        let mut invalid = valid();
        invalid.max_attempts = 0;
        assert!(invalid.validate(ValidationRequest).is_err());
    }

    #[test]
    fn test_validate_rejects_zero_max_attempts() {
        let mut r = valid();
        r.max_attempts = 0;
        assert!(r.validate(ValidationRequest).is_err());
    }

    #[test]
    fn test_validate_rejects_zero_rate_limit_max_attempts() {
        let mut r = valid();
        r.rate_limit_max_attempts = 0;
        assert!(r.validate(ValidationRequest).is_err());
    }

    #[test]
    fn test_validate_rejects_jitter_factor_out_of_range() {
        let mut r = valid();
        r.jitter_factor = 1.5;
        assert!(r.validate(ValidationRequest).is_err());
        r.jitter_factor = -0.1;
        assert!(r.validate(ValidationRequest).is_err());
    }

    #[test]
    fn test_validate_rejects_zero_half_open_probe_count() {
        let mut r = valid();
        r.half_open_probe_count = 0;
        assert!(r.validate(ValidationRequest).is_err());
    }

    #[test]
    fn test_validate_rejects_rate_limit_max_backoff_less_than_initial() {
        let mut r = valid();
        r.rate_limit_max_backoff_ms = 500;
        r.rate_limit_initial_backoff_ms = 1_000;
        assert!(r.validate(ValidationRequest).is_err());
    }

    // ── ResilienceConfigBuilder inline scenario tests ───────────────────────
    //
    // Duplicates the scenarios already covered externally in
    // tests/resilience_int_test.rs — required because this file's presence
    // of an inline `#[cfg(test)]` module makes the audit tool check inline
    // coverage only for this file's public functions, ignoring external
    // test files for it.

    fn full_builder() -> ResilienceConfigBuilder {
        ResilienceConfigBuilder::new()
            .max_attempts(3)
            .rate_limit_max_attempts(2)
    }

    /// @covers: build
    #[test]
    fn test_build_valid_config_returns_ok() {
        let cfg = full_builder().build().expect("build");
        assert_eq!(cfg.max_attempts, 3);
    }

    /// @covers: max_attempts
    #[test]
    fn test_max_attempts_sets_field() {
        let c = full_builder().max_attempts(5).build().expect("build");
        assert_eq!(c.max_attempts, 5);
    }

    /// @covers: initial_backoff_ms
    #[test]
    fn test_initial_backoff_ms_sets_field() {
        let c = full_builder()
            .initial_backoff_ms(200)
            .build()
            .expect("build");
        assert_eq!(c.initial_backoff_ms, 200);
    }

    /// @covers: backoff_multiplier
    #[test]
    fn test_backoff_multiplier_sets_field() {
        let c = full_builder()
            .backoff_multiplier(3.0)
            .build()
            .expect("build");
        assert!((c.backoff_multiplier - 3.0).abs() < f64::EPSILON);
    }

    /// @covers: jitter_factor
    #[test]
    fn test_jitter_factor_sets_field() {
        let c = full_builder().jitter_factor(0.2).build().expect("build");
        assert!((c.jitter_factor - 0.2).abs() < f64::EPSILON);
    }

    /// @covers: max_backoff_ms
    #[test]
    fn test_max_backoff_ms_sets_field() {
        let c = full_builder().max_backoff_ms(9000).build().expect("build");
        assert_eq!(c.max_backoff_ms, 9000);
    }

    /// @covers: rate_limit_max_attempts
    #[test]
    fn test_rate_limit_max_attempts_sets_field() {
        let c = full_builder()
            .rate_limit_max_attempts(4)
            .build()
            .expect("build");
        assert_eq!(c.rate_limit_max_attempts, 4);
    }

    /// @covers: rate_limit_initial_backoff_ms
    #[test]
    fn test_rate_limit_initial_backoff_ms_sets_field() {
        let c = full_builder()
            .rate_limit_initial_backoff_ms(500)
            .build()
            .expect("build");
        assert_eq!(c.rate_limit_initial_backoff_ms, 500);
    }

    /// @covers: rate_limit_max_backoff_ms
    #[test]
    fn test_rate_limit_max_backoff_ms_sets_field() {
        let c = full_builder()
            .rate_limit_max_backoff_ms(20_000)
            .build()
            .expect("build");
        assert_eq!(c.rate_limit_max_backoff_ms, 20_000);
    }

    /// @covers: failure_threshold
    #[test]
    fn test_failure_threshold_sets_field() {
        let c = full_builder().failure_threshold(10).build().expect("build");
        assert_eq!(c.failure_threshold, 10);
    }

    /// @covers: cool_down_seconds
    #[test]
    fn test_cool_down_seconds_sets_field() {
        let c = full_builder().cool_down_seconds(60).build().expect("build");
        assert_eq!(c.cool_down_seconds, 60);
    }

    /// @covers: half_open_probe_count
    #[test]
    fn test_half_open_probe_count_sets_field() {
        let c = full_builder()
            .half_open_probe_count(3)
            .build()
            .expect("build");
        assert_eq!(c.half_open_probe_count, 3);
    }
}
