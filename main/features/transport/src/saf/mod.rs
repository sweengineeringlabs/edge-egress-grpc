//! SAF layer — gRPC public facade.

mod factory;
mod processor;
mod validator;

pub use crate::api::client::tonic_grpc_client::TonicGrpcClient;
pub use factory::{create_config_builder, create_tonic_client_from_config, create_transport_from_config};
pub use processor::describe_processor;
pub use validator::validate_resilience_config;

pub use crate::api::interceptor::{
    GrpcEgressInterceptor, GrpcEgressInterceptorChain, TraceContextInterceptor,
};
pub use crate::api::port::{
    GrpcChannelConfigError, GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcMessageStream,
};
pub use crate::api::value_object::{
    CompressionMode, GrpcChannelConfig, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
    KeepAliveConfig, MtlsConfig, ResilienceConfig, ResilienceConfigBuilder,
    DEFAULT_MAX_MESSAGE_BYTES,
};
