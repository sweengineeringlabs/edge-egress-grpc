//! Types.

pub mod breaker_svc;

pub use breaker_svc::GrpcBreakerSvc;

pub mod grpc_breaker_client;
pub use grpc_breaker_client::GrpcBreakerClient;
