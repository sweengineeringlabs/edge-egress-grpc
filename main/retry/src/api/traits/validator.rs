//! `Validator` trait — configuration validation contract.

use crate::api::error::Error;
use crate::api::types::grpc::grpc_retry_config::GrpcRetryConfig;

/// Validates a [`GrpcRetryConfig`] for correctness.
///
/// Implemented by [`DefaultProcessor`](crate::core::processor::DefaultProcessor)
/// in `core/`.
pub trait Validator {
    /// Check that all numeric fields are within their valid ranges.
    fn validate_config(&self, config: &GrpcRetryConfig) -> Result<(), Error>;
}
