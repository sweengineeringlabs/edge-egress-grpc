//! SAF layer — public facade.

mod resilient_svc;

pub use crate::api::error::resilient_transport_error::ResilientTransportError;
pub use crate::api::types::{ApplicationConfigBuilder, GrpcResilientSvc};
