//! Interface counterpart for `core/resilience/resilient_grpc_client.rs`.
//!
//! [`ResilientGrpcClientPort`] is the api/ contract implemented by the
//! concrete `core::resilience::ResilientGrpcClient`.  Callers retrieve it
//! from the SAF layer as `Arc<dyn GrpcOutbound>` — this trait is the
//! documentation anchor and extension point.

use crate::api::port::{GrpcOutbound, GrpcOutboundError};

/// Extension contract for a gRPC client that adds resilience (retry + circuit breaker).
///
/// The concrete implementation lives in `core/resilience/`; consumers interact
/// with the type-erased `Arc<dyn GrpcOutbound>` surface returned by the SAF
/// factory functions.
#[allow(dead_code)]
pub trait ResilientGrpcClientPort: GrpcOutbound + Send + Sync {
    /// Return the current circuit-breaker state label for observability.
    ///
    /// Implementations must return one of: `"Closed"`, `"Open"`, `"HalfOpen"`.
    fn circuit_state(&self) -> &'static str;

    /// Return the count of consecutive post-retry failures tracked by the
    /// circuit breaker since it last closed.
    fn consecutive_failures(&self) -> u32;

    /// Expose the last transport error seen by the resilience layer, if any.
    ///
    /// Returns `None` when no failure has been recorded (circuit is `Closed`
    /// and no retry storms have fired).
    fn last_error(&self) -> Option<&GrpcOutboundError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resilient_grpc_client_port_is_object_safe() {
        fn _assert(_: &dyn ResilientGrpcClientPort) {}
    }

    #[test]
    fn test_grpc_outbound_re_export_is_object_safe() {
        fn _assert(_: &dyn GrpcOutbound) {}
    }
}
