//! `GrpcEgressError` — error type for gRPC outbound operations.

use thiserror::Error;

use crate::api::value_object::GrpcStatusCode;

/// Error type for gRPC outbound operations.
///
/// `Status(GrpcStatusCode, String)` is the canonical variant for a
/// well-formed gRPC reply that carried a non-`Ok` status code.  The
/// string is a *sanitized* message suitable to surface to callers and
/// must not contain server-side stack traces or internal paths.  See
/// `core::status_codes` for the wire ↔ enum mapping.
///
/// `ConnectionFailed`, `Timeout`, `Unavailable`, `Cancelled`, and
/// `Internal` are transport-level conditions that occur *before*
/// the server returns a status (or after the local timeout fires).
#[derive(Debug, Error)]
pub enum GrpcEgressError {
    /// The remote returned a non-`Ok` gRPC status with a sanitized message.
    #[error("status {0:?}: {1}")]
    Status(GrpcStatusCode, String),
    /// The transport could not establish a connection.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    /// The local per-call deadline elapsed before a response was received.
    #[error("timeout: {0}")]
    Timeout(String),
    /// An unexpected client-side condition.
    #[error("internal: {0}")]
    Internal(String),
    /// The remote endpoint was unavailable.
    #[error("unavailable: {0}")]
    Unavailable(String),
    /// The caller cancelled the in-flight request via the supplied token.
    #[error("cancelled: {0}")]
    Cancelled(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_variant_carries_code_and_message() {
        let err = GrpcEgressError::Status(GrpcStatusCode::NotFound, "no such row".into());
        let s = err.to_string();
        assert!(s.contains("NotFound"));
        assert!(s.contains("no such row"));
    }

    #[test]
    fn test_cancelled_variant_renders_with_reason() {
        let err = GrpcEgressError::Cancelled("token fired".into());
        assert!(err.to_string().contains("token fired"));
    }
}
