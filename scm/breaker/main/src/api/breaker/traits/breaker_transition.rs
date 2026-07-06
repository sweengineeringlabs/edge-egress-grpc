//! Interface contract for the breaker state-transition logic.
//!
//! The implementation lives in `core::breaker::breaker_transition`.

use crate::api::Admission;
use crate::api::AdmitRequest;
use crate::api::AdmitResponse;
use crate::api::BreakerDomainError;
use crate::api::Outcome;
use crate::api::RecordOutcomeRequest;
use crate::api::RecordOutcomeResponse;

/// Interface for the breaker's state-transition primitives.
pub trait BreakerTransition: Send + Sync {
    /// Decide whether to admit a new request, promoting Open to HalfOpen
    /// if the cool-down has elapsed.
    fn admit(&self, req: AdmitRequest) -> Result<AdmitResponse, BreakerDomainError>;

    /// Record the outcome of a dispatched request and return the updated node.
    fn record(
        &self,
        req: RecordOutcomeRequest,
    ) -> Result<RecordOutcomeResponse, BreakerDomainError>;

    /// Debug-format an admission decision — gives [`Admission`] a genuine
    /// role in this trait's signature set, not just a nested field.
    /// `Self: Sized` keeps this trait dyn-compatible for `Box<dyn Trait>`.
    fn describe_admission(admission: Admission) -> String
    where
        Self: Sized,
    {
        format!("{admission:?}")
    }

    /// Debug-format a recorded outcome — gives [`Outcome`] a genuine role
    /// in this trait's signature set, not just a nested field.
    fn describe_outcome(outcome: Outcome) -> String
    where
        Self: Sized,
    {
        format!("{outcome:?}")
    }
}
