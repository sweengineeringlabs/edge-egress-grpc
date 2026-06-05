//! Interface counterpart for `core/resilience/resilient_grpc_client.rs`.
//!
//! [`ResilientGrpcClientPort`] is the api/ contract implemented by the
//! concrete `core::resilience::ResilientGrpcClient`.  Callers retrieve it
//! from the SAF layer as `Arc<dyn GrpcEgress>` — this trait is the
//! documentation anchor and extension point.

use crate::api::error::GrpcEgressError;
use crate::api::traits::GrpcEgress;

/// Extension contract for a gRPC client that adds resilience (retry + circuit breaker).
///
/// The concrete implementation lives in `core/resilience/`; consumers interact
/// with the type-erased `Arc<dyn GrpcEgress>` surface returned by the SAF
/// factory functions.
pub trait ResilientGrpcClientPort: GrpcEgress + Send + Sync {
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
    fn last_error(&self) -> Option<&GrpcEgressError>;
}
