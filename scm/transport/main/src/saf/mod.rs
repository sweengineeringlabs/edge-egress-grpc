//! SAF layer — gRPC public facade.

// `impl TransportSvc` blocks only -- TransportSvc itself is declared in
// api/ and re-exported directly from lib.rs.
mod transport_svc;

mod grpc_egress_interceptor_svc_factory;
mod grpc_egress_svc_factory;
mod processor_svc_factory;
mod resilience_validator_svc_factory;
mod resilient_grpc_client_port_svc_factory;
mod validator_svc_factory;

pub use grpc_egress_interceptor_svc_factory::GrpcEgressInterceptorFactory;
pub use grpc_egress_svc_factory::GrpcEgressFactory;
pub use processor_svc_factory::ProcessorFactory;
pub use resilience_validator_svc_factory::ResilienceValidatorFactory;
pub use resilient_grpc_client_port_svc_factory::ResilientGrpcClientPortFactory;
pub use validator_svc_factory::ValidatorFactory;

#[cfg(feature = "prost")]
mod grpc_egress_prost_codec_svc_factory;
#[cfg(feature = "prost")]
pub use grpc_egress_prost_codec_svc_factory::GrpcEgressProstCodec;
