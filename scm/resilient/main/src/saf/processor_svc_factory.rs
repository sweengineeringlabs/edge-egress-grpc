//! Composition site for [`Processor`] — one file per trait keeps wiring focused.

use crate::api::{GrpcResilientSvc, Processor};

/// Factory for the default [`Processor`].
pub struct ProcessorFactory;

impl ProcessorFactory {
    /// Construct the default [`Processor`] — [`GrpcResilientSvc`], which
    /// identifies this crate's middleware as `"grpc-resilient"`.
    pub fn create() -> Box<dyn Processor> {
        Box::new(GrpcResilientSvc)
    }
}
