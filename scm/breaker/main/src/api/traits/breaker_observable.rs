//! Interface for observing circuit-breaker state from outside the decorator.

use futures::future::BoxFuture;

use crate::api::error::breaker_domain_error::BreakerDomainError;
use crate::api::types::observe_state_request::ObserveStateRequest;
use crate::api::types::observe_state_response::ObserveStateResponse;

/// Observability contract for a circuit-breaker decorator.
///
/// Implemented by [`crate::api::types::grpc_breaker_client::GrpcBreakerClient`]
/// in `core::breaker_observable`.
pub trait BreakerObservable: Send + Sync {
    /// Observe the current breaker state. Returns a snapshot; the breaker
    /// may transition immediately after this call returns.
    fn state(
        &self,
        req: ObserveStateRequest,
    ) -> BoxFuture<'_, Result<ObserveStateResponse, BreakerDomainError>>;
}
