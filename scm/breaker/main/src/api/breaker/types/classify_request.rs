//! Request for [`crate::api::FailureClassifier::classify`].

/// Input to [`crate::api::FailureClassifier::classify`] — whether the
/// dispatched call's result counts as a breaker-relevant failure signal.
///
/// The real inspection of `GrpcEgressError`/`GrpcStatusCode` (both foreign
/// to this crate) happens in `core::failure_classifier` before this
/// boolean is produced; this type keeps the trait boundary itself free of
/// foreign types.
pub struct ClassifyRequest {
    /// Whether the call counts as a breaker-relevant failure.
    pub is_breaker_failure: bool,
}
