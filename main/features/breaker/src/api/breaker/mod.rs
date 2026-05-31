//! Breaker sub-module — all circuit-breaker domain types.

pub(crate) mod admission;
pub(crate) mod breaker_state;
pub(crate) mod breaker_transition;
pub(crate) mod error;
pub(crate) mod failure_kind;
pub(crate) mod grpc_breaker_client;
pub(crate) mod grpc_breaker_config;
pub(crate) mod node;
pub(crate) mod outcome;

pub(crate) use admission::Admission;
pub use breaker_state::BreakerState;
pub(crate) use breaker_transition::BreakerTransition;
pub use error::Error;
pub(crate) use failure_kind::FailureClassifier;
pub use grpc_breaker_client::GrpcBreakerClient;
pub use grpc_breaker_config::GrpcBreakerConfig;
pub(crate) use node::BreakerNode;
pub(crate) use outcome::Outcome;
