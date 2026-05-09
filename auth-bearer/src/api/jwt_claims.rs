//! Interface counterpart for [`crate::core::jwt_claims`].
//!
//! `JwtClaims` is an internal value object used during token validation.
//! Consumers work with the interceptors via [`BearerInboundInterceptor`](super::bearer_inbound_interceptor::BearerInboundInterceptor)
//! and [`BearerOutboundInterceptor`](super::bearer_outbound_interceptor::BearerOutboundInterceptor).
