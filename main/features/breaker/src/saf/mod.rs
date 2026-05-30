//! SAF layer — public facade.

mod breaker_svc;

pub use crate::api::types::{GrpcBreakerClient, GrpcBreakerSvc};

pub use crate::api::breaker::config::GrpcBreakerConfig;
pub use crate::api::breaker::error::Error;
pub use crate::api::breaker::state::BreakerState;

pub use breaker_svc::{create_breaker_client, create_config_builder, wrap_breaker};
