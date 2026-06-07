//! SAF layer — gRPC public facade.

mod transport_svc;

pub use crate::api::traits::resilience::resilience_validator::ResilienceValidator;
pub use crate::api::traits::resilience::resilient_grpc_client_port::ResilientGrpcClientPort;
pub use crate::api::types::client::grpc_client_builder::GrpcClientBuilder;
pub use crate::api::types::status::conversions::Conversions;
pub use crate::api::types::ApplicationConfigBuilder;
pub use crate::api::types::TransportSvc;
pub use crate::spi::client::tonic::{TonicGrpcClient, TonicGrpcClientBuilder};

pub use crate::api::error::{GrpcChannelConfigError, GrpcEgressError};
pub use crate::api::traits::interceptor::grpc_egress_interceptor::GrpcEgressInterceptor;
pub use crate::api::traits::GrpcEgress;
pub use crate::api::types::grpc::grpc_channel_config_builder::GrpcChannelConfigBuilder;
pub use crate::api::types::grpc::grpc_request_builder::GrpcRequestBuilder;
pub use crate::api::types::interceptor::{
    GrpcEgressInterceptorChain, TraceContextInterceptor, TraceContextSource,
};
pub use crate::api::types::{
    CompressionMode, GrpcChannelConfig, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
    KeepAliveConfig, MtlsConfig, ResilienceConfig, ResilienceConfigBuilder,
    DEFAULT_MAX_MESSAGE_BYTES,
};
pub use crate::api::types::{GrpcEgressResult, GrpcMessageStream};
