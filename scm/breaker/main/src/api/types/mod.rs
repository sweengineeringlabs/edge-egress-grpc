//! Types — behavioural type declarations whose impl blocks live in `core/`.

pub mod application_config_builder;
pub(crate) mod breaker_node;
pub mod grpc_breaker_client;
pub mod grpc_breaker_svc;

pub use grpc_breaker_client::GrpcBreakerClient;
pub use grpc_breaker_svc::GrpcBreakerSvc;
