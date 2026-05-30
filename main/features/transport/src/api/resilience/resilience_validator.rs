//! Interface counterpart for `core/resilience/resilience_validator.rs`.
//!
//! [`ResilienceValidator`] is the public api/ interface for validating
//! resilience configuration before it is applied to a transport channel.
//!
//! The concrete implementation lives in `core/resilience/` and is wired
//! through the SAF layer.

use crate::api::value::ResilienceConfig;

/// Validates a [`ResilienceConfig`] before it is applied to a gRPC channel.
///
/// Returns `Err` with a human-readable description when the configuration
/// contains values that would produce unsafe or undefined retry / breaker
/// behaviour (e.g. `max_attempts == 0`, `backoff_multiplier <= 0.0`).
pub trait ResilienceValidator: Send + Sync {
    /// Validate `config` and return `Err(reason)` on the first violation.
    fn validate_config(&self, config: &ResilienceConfig) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::traits::Validator;

    #[test]
    fn test_resilience_validator_is_object_safe() {
        fn _assert(_: &dyn ResilienceValidator) {}
    }

    #[test]
    fn test_validator_re_export_is_object_safe() {
        fn _assert(_: &dyn Validator) {}
    }
}
