//! API layer — public schema + trait declarations + public types.
//!
//! Per SEA rule 160, public type *declarations* live here.  Impl
//! blocks live in `core/`.

mod error;
mod traits;
mod types;

pub use error::{BreakerDomainError, Error};

pub use traits::{
    BreakerDecorator, BreakerObservable, ConfigBuilderProvider, Processor, Validator,
};

pub use types::{
    ApplicationConfigBuilder, BreakerState, ConfigBuilderRequest, ConfigValidationRequest,
    DescribeRequest, DescribeResponse, GrpcBreakerClient, GrpcBreakerConfig, GrpcBreakerSvc,
    ObserveStateRequest, ObserveStateResponse, WrapBreakerRequest,
};

pub(crate) use traits::BreakerTransition;

pub(crate) use types::{
    Admission, AdmitRequest, AdmitResponse, BreakerNode, Outcome, RecordOutcomeRequest,
    RecordOutcomeResponse,
};
