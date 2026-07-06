//! SEA interface contract — primary traits for `swe-edge-egress-grpc`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |

pub use crate::api::traits::processor::Processor;

use crate::api::{GrpcChannelConfigError, ValidationRequest};

/// Configuration validation contract.
///
/// Implemented by configuration types (e.g. [`crate::api::types::ResilienceConfig`])
/// to validate their fields before use.
pub trait Validator: Send + Sync {
    /// Validate the configuration.
    ///
    /// Returns `Err` when the configuration contains an invalid combination
    /// of fields.
    fn validate(&self, req: ValidationRequest) -> Result<(), GrpcChannelConfigError>;
}
