//! Default implementation of [`Processor`] and [`Validator`] for
//! the retry crate.
//!
//! [`DefaultProcessor`] validates a [`GrpcRetryConfig`] for correctness.

use crate::api::error::Error;
use crate::api::traits::{Processor, Validator};
use crate::api::types::grpc_retry_config::GrpcRetryConfig;

/// Default [`Processor`] and [`Validator`] implementation for the
/// gRPC retry crate.
pub(crate) struct DefaultProcessor;

impl Processor for DefaultProcessor {
    fn validate(&self, config: &GrpcRetryConfig) -> Result<(), Error> {
        config.validate()
    }
}

impl Validator for DefaultProcessor {
    fn validate_config(&self, config: &GrpcRetryConfig) -> Result<(), Error> {
        config.validate()
    }
}
