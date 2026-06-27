//! Types — behavioural type declarations whose impl blocks live in `core/`.

pub(crate) mod admission;
pub mod application_config_builder;
pub(crate) mod breaker_node;
pub(crate) mod breaker_state;
pub mod grpc_breaker_client;
pub mod grpc_breaker_config;
pub mod grpc_breaker_svc;
pub(crate) mod outcome;

pub use breaker_state::BreakerState;
pub use grpc_breaker_client::GrpcBreakerClient;
