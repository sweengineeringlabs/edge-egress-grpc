//! SEA interface contract — primary traits for `swe-edge-egress-grpc`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |

pub use crate::api::traits::processor::Processor;

/// Configuration validation contract.
///
/// Implemented by configuration types (e.g. [`crate::api::vo::ResilienceConfig`])
/// to validate their fields before use.
pub trait Validator {
    /// Validate the configuration.
    ///
    /// Returns `Err` with a human-readable description when the configuration
    /// contains an invalid combination of fields.
    fn validate(&self) -> Result<(), String>;
}
