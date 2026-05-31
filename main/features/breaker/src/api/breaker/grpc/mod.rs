//! gRPC breaker types.

pub mod grpc_breaker_client;
pub mod grpc_breaker_config;

pub use grpc_breaker_client::GrpcBreakerClient;
pub use grpc_breaker_config::GrpcBreakerConfig;
