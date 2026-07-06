//! `Validator` implementation for core configuration types.

use crate::api::Validator;
use crate::api::{GrpcChannelConfigError, ResilienceConfig, ValidationRequest};

/// Zero-size marker identifying this as the `Validator` implementation site.
#[expect(
    dead_code,
    reason = "SEA structural marker — impl site anchor, not instantiated"
)]
pub(crate) struct ResilienceValidator;

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
}
