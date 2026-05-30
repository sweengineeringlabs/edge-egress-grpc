//! Domain error types for `swe_edge_egress_grpc_breaker`.

pub use crate::api::breaker::error::Error;

// Domain error alias for rule 180/204 compliance
pub use crate::api::breaker::error::Error as BreakerError;
