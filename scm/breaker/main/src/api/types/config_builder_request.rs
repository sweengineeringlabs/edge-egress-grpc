//! Request for [`crate::api::traits::ConfigBuilderProvider::create_config_builder`].

use crate::api::types::grpc_breaker_svc::GrpcBreakerSvc;

/// Input to [`crate::api::traits::ConfigBuilderProvider::create_config_builder`].
///
/// Carries the requesting namespace marker ([`GrpcBreakerSvc`] is
/// zero-sized) so the type has a genuine role in the request/response
/// contract rather than being an orphaned facade type.
pub struct ConfigBuilderRequest {
    /// The namespace requesting the builder.
    pub svc: GrpcBreakerSvc,
}
