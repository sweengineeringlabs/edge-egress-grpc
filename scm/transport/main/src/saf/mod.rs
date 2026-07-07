//! SAF layer — gRPC public facade.

// `impl TransportSvc` blocks only -- TransportSvc itself is declared in
// api/ and re-exported directly from lib.rs.
mod transport_svc;

mod grpc;
mod processor_svc_factory;
mod resilience_validator_svc_factory;
mod resilient_grpc_client_port_svc_factory;
mod validator_svc_factory;

pub use grpc::GrpcEgressFactory;
pub use grpc::GrpcEgressInterceptorFactory;
pub use processor_svc_factory::ProcessorFactory;
pub use resilience_validator_svc_factory::ResilienceValidatorFactory;
pub use resilient_grpc_client_port_svc_factory::ResilientGrpcClientPortFactory;
pub use validator_svc_factory::ValidatorFactory;

#[cfg(feature = "prost")]
pub use grpc::GrpcEgressProstCodec;
