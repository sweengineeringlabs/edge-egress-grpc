//! SEA interface contract — primary traits for `swe-edge-egress-grpc`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |

pub use crate::api::processor::Processor;

/// Configuration validation contract.
///
/// Implemented by configuration types (e.g. [`crate::api::value_object::ResilienceConfig`])
/// to validate their fields before use.
pub trait Validator {
    /// Validate the configuration.
    ///
    /// Returns `Err` with a human-readable description when the configuration
    /// contains an invalid combination of fields.
    fn validate(&self) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::port::GrpcEgress;

    #[test]
    fn test_grpc_egress_re_export_is_object_safe() {
        fn _assert(_: &dyn GrpcEgress) {}
    }

    #[test]
    fn test_processor_is_object_safe() {
        fn _assert(_: &dyn Processor) {}
    }

    #[test]
    fn test_validator_is_object_safe() {
        fn _assert(_: &dyn Validator) {}
    }
}
