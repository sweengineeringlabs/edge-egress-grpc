//! SEA interface contract — primary traits for `swe-edge-egress-grpc-resilient`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |

pub mod processor;
pub use processor::Processor;

/// Configuration validation contract for the resilient transport.
pub trait Validator: Send + Sync {
    /// Validate the given channel configuration.
    ///
    /// Returns `Ok(())` when valid; an error string describing the violation otherwise.
    fn validate(&self, config: &swe_edge_egress_grpc::GrpcChannelConfig) -> Result<(), String>;
}
