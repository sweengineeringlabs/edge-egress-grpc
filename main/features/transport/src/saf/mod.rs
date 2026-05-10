//! SAF layer — gRPC public facade.

mod factory;
mod validator;

pub use crate::api::client::tonic_grpc_client::TonicGrpcClient;
pub use factory::create_transport_from_config;
pub use validator::validate_resilience_config;

pub use crate::api::interceptor::{
    GrpcOutboundInterceptor, GrpcOutboundInterceptorChain, TraceContextInterceptor,
};
pub use crate::api::port::{
    GrpcChannelConfigError, GrpcMessageStream, GrpcOutbound, GrpcOutboundError, GrpcOutboundResult,
};
pub use crate::api::value_object::{
    CompressionMode, GrpcChannelConfig, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
    KeepAliveConfig, MtlsConfig, ResilienceConfig, ResilienceConfigBuilder,
    DEFAULT_MAX_MESSAGE_BYTES,
};
