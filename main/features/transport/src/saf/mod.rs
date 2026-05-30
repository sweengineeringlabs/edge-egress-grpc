//! SAF layer — gRPC public facade.

mod transport_svc;

pub use transport_svc::{
    create_config_builder, create_tonic_client_from_config, create_transport_from_config,
    describe_processor, validate_resilience_config,
};

pub use crate::api::types::{TonicGrpcClient, TransportSvc};

pub use crate::api::interceptor::{
    GrpcEgressInterceptor, GrpcEgressInterceptorChain, TraceContextInterceptor,
};
pub use crate::api::port::{
    GrpcChannelConfigError, GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcMessageStream,
};
pub use crate::api::value::{
    CompressionMode, GrpcChannelConfig, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
    KeepAliveConfig, MtlsConfig, ResilienceConfig, ResilienceConfigBuilder,
    DEFAULT_MAX_MESSAGE_BYTES,
};
