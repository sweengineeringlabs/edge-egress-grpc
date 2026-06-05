//! Primary trait contracts for `swe-edge-egress-grpc-transport`.

#[allow(clippy::module_inception)]
pub mod traits;
pub use traits::{Processor, Validator};

pub mod grpc_egress;
pub use grpc_egress::GrpcEgress;

pub mod interceptor;
pub mod processor;
pub mod resilience;
