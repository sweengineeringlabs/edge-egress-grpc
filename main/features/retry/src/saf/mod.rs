//! SAF layer — public facade.

mod retry_svc;

pub use crate::api::error::Error;
pub use crate::api::types::{
    BackoffSchedule, GrpcRetryClient, GrpcRetryConfig, GrpcRetryConfigBuilder, GrpcRetrySvc,
    ResourceExhaustedContext, RetryDecision,
};

pub use retry_svc::{create_config_builder, create_retry_client, wrap_retry};
