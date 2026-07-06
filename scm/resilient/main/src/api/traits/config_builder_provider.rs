//! Interface for obtaining a pre-seeded application config builder.

use crate::api::ConfigBuilderRequest;
use crate::api::ConfigBuilderResponse;
use crate::api::GrpcResilientSvc;
use crate::api::ResilientTransportError;

/// Contract for producing a config builder pre-populated with this crate's
/// name and version.
///
/// Implemented by [`GrpcResilientSvc`] in `core/`.
pub trait ConfigBuilderProvider: Send + Sync {
    /// Create a config builder pre-populated with this crate's name and version.
    fn create_config_builder(
        &self,
        req: ConfigBuilderRequest,
    ) -> Result<ConfigBuilderResponse, ResilientTransportError>;

    /// Construct the default provider — gives [`GrpcResilientSvc`] a genuine
    /// role in this trait's signature set, not just an impl-site `Self`.
    /// `Self: Sized` keeps this trait dyn-compatible for `Box<dyn Trait>`.
    fn default_provider() -> GrpcResilientSvc
    where
        Self: Sized,
    {
        GrpcResilientSvc
    }
}
