//! Request for [`crate::api::traits::BreakerDecorator::wrap`].

use crate::api::types::grpc_breaker_config::GrpcBreakerConfig;

/// Input to [`crate::api::traits::BreakerDecorator::wrap`] — the inner
/// client to decorate and the breaker policy to enforce around it.
pub struct WrapBreakerRequest<T> {
    /// The inner client being wrapped.
    pub inner: T,
    /// The breaker policy to enforce.
    pub config: GrpcBreakerConfig,
}
