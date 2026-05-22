//! Interface counterpart for [`crate::core::jwt_claims`].
//!
//! `JwtClaims` is an internal value object used during token validation.
//! Consumers work with the interceptors via [`BearerIngressInterceptor`](super::bearer_ingress_interceptor::BearerIngressInterceptor)
//! and [`BearerEgressInterceptor`](super::bearer_egress_interceptor::BearerEgressInterceptor).
