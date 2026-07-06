//! SAF layer — gRPC public facade.

// `impl TransportSvc` blocks only -- TransportSvc itself is declared in
// api/ and re-exported directly from lib.rs.
mod transport_svc;

#[cfg(feature = "prost")]
mod grpc_egress_prost_codec;
#[cfg(feature = "prost")]
pub use grpc_egress_prost_codec::GrpcEgressProstCodec;
