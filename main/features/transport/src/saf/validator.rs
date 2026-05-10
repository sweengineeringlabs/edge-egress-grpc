//! SAF wrapper for the `Validator` trait on resilience configuration.

use crate::api::traits::Validator;
use crate::api::value_object::ResilienceConfig;

/// Validate a [`ResilienceConfig`] using the [`Validator`] contract.
///
/// Returns `Err` with a human-readable description when the configuration
/// contains an invalid combination of fields.
pub fn validate_resilience_config(config: &ResilienceConfig) -> Result<(), String> {
    config.validate()
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

    /// @covers: validate_resilience_config
    #[test]
    fn test_validate_resilience_config_valid_returns_ok() {
        assert!(validate_resilience_config(&valid()).is_ok());
    }

    /// @covers: validate_resilience_config
    #[test]
    fn test_validate_resilience_config_invalid_returns_err() {
        let mut r = valid();
        r.max_attempts = 0;
        assert!(validate_resilience_config(&r).is_err());
    }
}
