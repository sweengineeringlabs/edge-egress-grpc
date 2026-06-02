//! Breaker sub-module — all circuit-breaker domain types.

pub(crate) mod admission;
pub(crate) mod breaker_state;
pub(crate) mod breaker_transition;
pub(crate) mod client;
pub(crate) mod error;
pub(crate) mod failure_kind;
pub(crate) mod grpc;
pub(crate) mod node;
pub(crate) mod outcome;

pub use breaker_state::BreakerState;
pub use error::Error;
pub use grpc::GrpcBreakerClient;
pub use grpc::GrpcBreakerConfig;
