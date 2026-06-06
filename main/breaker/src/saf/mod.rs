//! SAF layer — public facade.

mod breaker_svc;

pub use crate::api::types::{GrpcBreakerClient, GrpcBreakerSvc};

pub use crate::api::error::Error;
pub use crate::api::vo::{BreakerState, GrpcBreakerConfig};
