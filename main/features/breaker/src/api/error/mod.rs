//! Domain error types for `swe_edge_egress_grpc_breaker`.

pub mod error;
pub use error::Error;
pub mod domain_error;
pub use domain_error::BreakerDomainError;
