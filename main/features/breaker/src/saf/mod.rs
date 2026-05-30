//! SAF layer — public facade.

mod breaker_svc;

pub use crate::api::types::{GrpcBreakerClient, GrpcBreakerSvc};

pub use crate::api::breaker::breaker_state::BreakerState;
pub use crate::api::breaker::error::Error;
pub use crate::api::breaker::grpc_breaker_config::GrpcBreakerConfig;
