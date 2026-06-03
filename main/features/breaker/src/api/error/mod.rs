//! Domain error types for `swe_edge_egress_grpc_breaker`.

pub mod breaker_domain_error;
#[allow(clippy::module_inception)]
pub mod error;
pub use error::Error;
