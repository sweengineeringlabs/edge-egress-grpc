//! SAF layer — public facade.

mod resilient_svc;

pub use crate::api::ResilientTransportError;
pub use crate::api::{ApplicationConfigBuilder, GrpcResilientSvc};
