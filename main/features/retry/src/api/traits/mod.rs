//! Primary trait declarations for `swe_edge_egress_grpc_retry`.
//!
//! This crate's external port contract is
//! [`GrpcEgress`](swe_edge_egress_grpc::GrpcEgress) — declared
//! upstream in `swe-edge-egress-grpc`. Consumers should depend on
//! that crate directly for the trait; this crate's job is to wrap
//! implementors with retry logic, not to re-publish the contract.

use crate::api::error::Error;
use crate::api::types::grpc_retry_config::GrpcRetryConfig;

/// Processes a retry decision given a gRPC result.
///
/// Implemented by [`DefaultProcessor`](crate::core::processor::DefaultProcessor)
/// in `core/`.
pub trait Processor {
    /// Validate the retry configuration.
    ///
    /// Returns `Ok(())` when the configuration is valid, or
    /// [`Error::InvalidConfig`] when a field is out of range.
    fn validate(&self, config: &GrpcRetryConfig) -> Result<(), Error>;
}

/// Validates a [`GrpcRetryConfig`] for correctness.
///
/// Implemented by [`DefaultProcessor`](crate::core::processor::DefaultProcessor)
/// in `core/`.
pub trait Validator {
    /// Check that all numeric fields are within their valid ranges.
    fn validate_config(&self, config: &GrpcRetryConfig) -> Result<(), Error>;
}
