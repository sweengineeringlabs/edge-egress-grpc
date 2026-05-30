//! `swe_edge_egress_grpc` — gRPC outbound domain.

mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
