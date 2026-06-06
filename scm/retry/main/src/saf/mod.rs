//! SAF layer — public facade.

mod retry_svc;

pub use crate::api::error::Error;
pub use crate::api::types::{GrpcRetryClient, GrpcRetrySvc};
pub use crate::api::vo::{
    BackoffSchedule, GrpcRetryConfig, GrpcRetryConfigBuilder, ResourceExhaustedContext,
    RetryDecision,
};
