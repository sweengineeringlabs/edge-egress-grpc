//! Classification of [`GrpcEgressError`] values into the
//! breaker's binary success/failure outcome.
//!
//! Single source of truth for which gRPC errors trip the breaker.
//!
//! ## Counted as failures
//!
//! - `GrpcEgressError::Unavailable(_)`
//! - `GrpcEgressError::ConnectionFailed(_)`
//! - `GrpcEgressError::Internal(_)`
//! - `GrpcEgressError::Status(Unavailable, _)`
//! - `GrpcEgressError::Status(Internal, _)`
//!
//! ## NOT counted as failures
//!
//! - `Status(ResourceExhausted, _)` — rate-limit signal, not an
//!   unhealthy upstream.  Retry layer handles it.
//! - `Status(Unauthenticated, _)` / `Status(PermissionDenied, _)` —
//!   credentials are wrong; the upstream is fine.  Tripping the
//!   breaker on auth failures would shed traffic that has nothing
//!   to do with upstream health.
//! - `Status(NotFound, _)`, `InvalidArgument`, `FailedPrecondition`,
//!   etc. — caller-visible business errors.  Upstream working as
//!   intended.
//! - `Timeout(_)` — caller's own deadline, not a breaker condition.
//! - `Cancelled(_)` — caller cancelled.

use swe_edge_egress_grpc::{GrpcEgressError, GrpcStatusCode};

use crate::api::breaker::outcome::Outcome;

/// Classify an outbound result into the breaker's outcome.
pub(crate) fn classify<T>(result: &Result<T, GrpcEgressError>) -> Outcome {
    let err = match result {
        Ok(_) => return Outcome::Success,
        Err(e) => e,
    };
    match err {
        GrpcEgressError::Unavailable(_)
        | GrpcEgressError::ConnectionFailed(_)
        | GrpcEgressError::Internal(_) => Outcome::Failure,

        GrpcEgressError::Status(code, _) => match code {
            GrpcStatusCode::Unavailable | GrpcStatusCode::Internal => Outcome::Failure,
            // Explicit non-failure variants — exhaustive so a
            // new variant on `GrpcStatusCode` causes a compile
            // error here, not a silent default.
            GrpcStatusCode::Ok
            | GrpcStatusCode::Cancelled
            | GrpcStatusCode::Unknown
            | GrpcStatusCode::InvalidArgument
            | GrpcStatusCode::DeadlineExceeded
            | GrpcStatusCode::NotFound
            | GrpcStatusCode::AlreadyExists
            | GrpcStatusCode::PermissionDenied
            | GrpcStatusCode::ResourceExhausted
            | GrpcStatusCode::FailedPrecondition
            | GrpcStatusCode::Aborted
            | GrpcStatusCode::OutOfRange
            | GrpcStatusCode::Unimplemented
            | GrpcStatusCode::DataLoss
            | GrpcStatusCode::Unauthenticated => Outcome::Success,
        },

        GrpcEgressError::Timeout(_) | GrpcEgressError::Cancelled(_) => Outcome::Success,
    }
}

/// Stateless classifier for gRPC egress results.
pub(crate) struct FailureClassifier;

impl FailureClassifier {
    /// Classify an outbound result into the breaker's outcome.
    pub(crate) fn classify<T>(result: &Result<T, GrpcEgressError>) -> Outcome {
        classify(result)
    }
}
