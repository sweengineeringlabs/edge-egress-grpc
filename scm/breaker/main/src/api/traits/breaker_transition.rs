//! Interface contract for the breaker state-transition logic.
//!
//! The implementation lives in `core::breaker_transition`.

use crate::api::error::breaker_domain_error::BreakerDomainError;
use crate::api::types::admit_request::AdmitRequest;
use crate::api::types::admit_response::AdmitResponse;
use crate::api::types::record_outcome_request::RecordOutcomeRequest;
use crate::api::types::record_outcome_response::RecordOutcomeResponse;

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
}
