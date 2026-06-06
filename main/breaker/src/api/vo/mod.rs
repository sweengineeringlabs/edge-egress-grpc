//! Value objects — breaker policy schema and state-machine values.

pub(crate) mod admission;
pub(crate) mod breaker_state;
pub(crate) mod grpc_breaker_config;
pub(crate) mod outcome;

pub use breaker_state::BreakerState;
pub use grpc_breaker_config::GrpcBreakerConfig;
