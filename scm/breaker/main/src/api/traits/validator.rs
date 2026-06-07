//! `Validator` trait — configuration validation contract.

use crate::api::error::error::Error;
use crate::api::types::grpc_breaker_config::GrpcBreakerConfig;

/// Validation contract for circuit-breaker configuration.
pub trait Validator: Send + Sync {
    /// Validate the breaker configuration.
    fn validate(&self, config: &GrpcBreakerConfig) -> Result<(), Error>;
}
