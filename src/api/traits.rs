//! SEA interface contract — primary traits for `swe-edge-egress-grpc`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`GrpcOutbound`] | Makes outbound unary and streaming gRPC calls |
//! | [`GrpcOutboundInterceptor`] | Observes/mutates requests before and after dispatch |
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |

pub use crate::api::port::GrpcOutbound;
pub use crate::api::interceptor::GrpcOutboundInterceptor;
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

    #[test]
    fn test_grpc_outbound_re_export_is_object_safe() {
        fn _assert(_: &dyn GrpcOutbound) {}
    }

    #[test]
    fn test_validator_is_object_safe() {
        fn _assert(_: &dyn Validator) {}
    }
}
