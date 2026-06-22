//! `swe_edge_egress_grpc` — gRPC outbound domain.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::*;
// Re-export key types for dependents
pub use api::error::GrpcChannelConfigError;
pub use api::error::GrpcEgressError;
pub use api::traits::GrpcEgress;
pub use api::types::GrpcChannelConfig;
pub use api::types::TransportSvc;
