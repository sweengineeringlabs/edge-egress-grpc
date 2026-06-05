//! Domain error types for `swe_edge_egress_grpc_transport`.

pub mod grpc_channel_config_error;
pub mod grpc_egress_error;
pub use grpc_channel_config_error::GrpcChannelConfigError;
pub use grpc_egress_error::GrpcEgressError;
