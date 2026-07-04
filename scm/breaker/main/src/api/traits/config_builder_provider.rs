//! Interface for obtaining a pre-seeded application config builder.

use crate::api::error::breaker_domain_error::BreakerDomainError;
use crate::api::types::application_config_builder::ApplicationConfigBuilder;
use crate::api::types::config_builder_request::ConfigBuilderRequest;

/// Contract for producing a config builder pre-populated with this crate's
/// name and version.
///
/// Implemented by [`crate::api::types::grpc_breaker_svc::GrpcBreakerSvc`] in
/// `core::config_builder_provider`.
pub trait ConfigBuilderProvider: Send + Sync {
    /// Create a config builder pre-populated with this crate's name and version.
    fn create_config_builder(
        &self,
        req: ConfigBuilderRequest,
    ) -> Result<ApplicationConfigBuilder, BreakerDomainError>;
}
