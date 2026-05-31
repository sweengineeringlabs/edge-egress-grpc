//! Domain error types for `swe_edge_egress_grpc_breaker`.

pub mod error;
pub use error::Error;
pub mod breaker_domain_error;
pub use breaker_domain_error::BreakerDomainError;
