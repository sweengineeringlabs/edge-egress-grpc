//! Primary trait declarations for `swe_edge_egress_grpc_retry`.

pub mod processor;
pub mod validator;

pub use processor::Processor;
pub use validator::Validator;
