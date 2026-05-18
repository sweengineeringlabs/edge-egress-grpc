//! Error type for the resilient transport factory.

use swe_edge_egress_grpc::GrpcChannelConfigError;

/// Error produced by [`crate::create_resilient_transport_from_config`].
#[derive(Debug, thiserror::Error)]
pub enum ResilientTransportError {
    /// The base channel configuration is invalid (e.g. plaintext rejected).
    #[error(transparent)]
    ChannelConfig(#[from] GrpcChannelConfigError),
    /// The resilience policy contains an invalid field combination.
    #[error("invalid resilience config: {0}")]
    InvalidResilience(String),
}
