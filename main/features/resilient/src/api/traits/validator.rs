//! `Validator` trait — configuration validation contract.

/// Configuration validation contract for the resilient transport.
pub trait Validator: Send + Sync {
    /// Validate the given channel configuration.
    ///
    /// Returns `Ok(())` when valid; an error string describing the violation otherwise.
    fn validate(&self, config: &swe_edge_egress_grpc::GrpcChannelConfig) -> Result<(), String>;
}
