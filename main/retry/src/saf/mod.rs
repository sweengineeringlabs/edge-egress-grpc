//! SAF layer — public facade.

mod retry_svc;

pub use crate::api::error::Error;
pub use crate::api::types::{
    BackoffSchedule, GrpcRetryClient, GrpcRetryConfig, GrpcRetryConfigBuilder, GrpcRetrySvc,
    ResourceExhaustedContext, RetryDecision,
};
