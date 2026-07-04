//! Interface for constructing circuit-breaker decorators.

use crate::api::error::breaker_domain_error::BreakerDomainError;
use crate::api::types::grpc_breaker_client::GrpcBreakerClient;
use crate::api::types::wrap_breaker_request::WrapBreakerRequest;

/// Contract for wrapping an inner client with a circuit-breaker policy.
///
/// The concrete implementation lives in `core::breaker_decorator`; callers
/// obtain a decorator via the `saf::breaker_decorator_svc_factory`.
pub trait BreakerDecorator<T>: Send + Sync {
    /// Wrap `inner` with the supplied breaker policy, returning the decorator.
    fn wrap(&self, req: WrapBreakerRequest<T>) -> Result<GrpcBreakerClient<T>, BreakerDomainError>;
}
