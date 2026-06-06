//! `Processor` trait — primary processing contract for the retry crate.

use crate::api::error::Error;
use crate::api::vo::grpc_retry_config::GrpcRetryConfig;

/// Processes a retry decision given a gRPC result.
///
/// Implemented by [`DefaultProcessor`](crate::core::default_processor::DefaultProcessor)
/// in `core/`.
pub trait Processor {
    /// Validate the retry configuration.
    ///
    /// Returns `Ok(())` when the configuration is valid, or
    /// [`Error::InvalidConfig`] when a field is out of range.
    fn validate(&self, config: &GrpcRetryConfig) -> Result<(), Error>;
}
