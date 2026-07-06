//! API layer — public schema + trait declarations + public types.
//!
//! Per SEA rule 160, public type *declarations* live here.  Impl
//! blocks live in `core/`.

mod error;
mod traits;
mod types;

pub use error::Error;
pub use traits::{ConfigBuilderProvider, JitterRng, Processor, Validator};
pub use types::{
    ApplicationConfigBuilder, BackoffSchedule, ConfigBuilderRequest, ConfigBuilderResponse,
    GrpcRetryClient, GrpcRetryConfig, GrpcRetryConfigBuilder, GrpcRetryFacade, GrpcRetrySvc,
    NextUnitRequest, NextUnitResponse, ProcessorRequest, ResourceExhaustedContext, RetryDecision,
    ValidationRequest,
};
