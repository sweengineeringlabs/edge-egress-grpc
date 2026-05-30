//! Breaker sub-module — all circuit-breaker domain types.

pub(crate) mod admission;
pub(crate) mod client;
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod failure_kind;
pub(crate) mod node;
pub(crate) mod outcome;
pub(crate) mod transitions;

pub(crate) use admission::Admission;
pub use client::GrpcBreakerClient;
pub use config::GrpcBreakerConfig;
pub use error::Error;
pub(crate) use failure_kind::classify;
pub(crate) use node::BreakerNode;
pub(crate) use outcome::Outcome;
pub use state::BreakerState;
pub(crate) use transitions::BreakerTransitions;

pub(crate) mod state;
