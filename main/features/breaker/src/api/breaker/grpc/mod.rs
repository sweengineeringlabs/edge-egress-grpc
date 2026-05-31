//! gRPC breaker types.

pub mod breaker_client;
pub mod breaker_config;

pub use breaker_client::GrpcBreakerClient;
pub use breaker_config::GrpcBreakerConfig;
