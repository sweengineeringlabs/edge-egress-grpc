//! `Validator` trait — configuration validation contract.

use crate::api::breaker::config::GrpcBreakerConfig;
use crate::api::breaker::error::Error;

/// Validation contract for circuit-breaker configuration.
pub trait Validator: Send + Sync {
    /// Validate the breaker configuration.
    fn validate(&self, config: &GrpcBreakerConfig) -> Result<(), Error>;
}
