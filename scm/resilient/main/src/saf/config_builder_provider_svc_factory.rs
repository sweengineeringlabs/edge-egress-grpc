//! Composition site for [`ConfigBuilderProvider`] — one file per trait keeps wiring focused.

use crate::api::{ConfigBuilderProvider, GrpcResilientSvc};

/// Factory for the default [`ConfigBuilderProvider`].
pub struct ConfigBuilderProviderFactory;

impl ConfigBuilderProviderFactory {
    /// Construct the default [`ConfigBuilderProvider`] — [`GrpcResilientSvc`].
    pub fn create() -> Box<dyn ConfigBuilderProvider> {
        Box::new(GrpcResilientSvc)
    }
}
