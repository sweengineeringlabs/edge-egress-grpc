//! `Validator` trait — configuration validation contract.

use crate::api::error::breaker_domain_error::BreakerDomainError;
use crate::api::types::config_validation_request::ConfigValidationRequest;

/// Validation contract for circuit-breaker configuration.
pub trait Validator: Send + Sync {
    /// Validate the breaker configuration.
    fn validate(&self, req: ConfigValidationRequest) -> Result<(), BreakerDomainError>;
}
