//! `swe_edge_egress_grpc` — gRPC outbound domain.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
