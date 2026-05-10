//! SAF layer — public facade.
//!
//! Single source of public re-exports.  api/ and core/ stay
//! crate-private; consumers only see what we surface here.

mod builder;

pub use crate::api::backoff::BackoffSchedule;
pub use crate::api::error::Error;
pub use crate::api::retry_client::GrpcRetryClient;
pub use crate::api::retry_config::GrpcRetryConfig;
pub use crate::api::retry_policy::{
    classify, classify_resource_exhausted, parse_retry_after_hint, ResourceExhaustedContext,
    RetryDecision,
};
pub use builder::{builder, create_retry_client, Builder};
