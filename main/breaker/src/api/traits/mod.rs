//! Primary trait declarations for `swe_edge_egress_grpc_breaker`.

pub(crate) mod breaker_egress;
pub(crate) mod breaker_transition;
pub mod processor;
pub mod validator;

pub use processor::Processor;
pub use validator::Validator;
