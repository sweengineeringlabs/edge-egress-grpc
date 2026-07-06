//! Interface for classifying dispatch results into the breaker's binary
//! success/failure outcome — the api/ counterpart to `core::
//! failure_classifier`.

use crate::api::{BreakerDomainError, ClassifyRequest, ClassifyResponse};

/// Classification contract — the real inspection of `GrpcEgressError`/
/// `GrpcStatusCode` (both foreign to this crate) lives in
/// `core::failure_classifier`; this trait only sees the resulting boolean.
pub trait FailureClassifier: Send + Sync {
    /// Classify a breaker-failure signal into the breaker's outcome.
    fn classify(&self, req: ClassifyRequest) -> Result<ClassifyResponse, BreakerDomainError>;
}
