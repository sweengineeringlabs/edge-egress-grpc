//! SAF layer — gRPC public facade.

mod transport_svc;

#[cfg(feature = "prost")]
mod grpc_egress_prost_codec;
#[cfg(feature = "prost")]
pub use grpc_egress_prost_codec::GrpcEgressProstCodec;

pub use transport_svc::*;
