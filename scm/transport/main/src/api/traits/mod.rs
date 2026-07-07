//! Primary trait contracts for `swe-edge-egress-grpc-transport`.

#[allow(clippy::module_inception)]
pub mod traits;
pub use traits::{Processor, Validator};

pub mod grpc_egress;
pub use grpc_egress::GrpcEgress;

pub mod grpc_egress_interceptor;
pub mod processor;
pub mod resilience_validator;
pub mod resilient_grpc_client_port;

#[cfg(feature = "prost")]
pub mod grpc_egress_prost_codec;
#[cfg(feature = "prost")]
pub use grpc_egress_prost_codec::GrpcEgressProstCodec;
