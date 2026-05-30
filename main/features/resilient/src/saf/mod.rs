//! SAF layer — public facade.

mod resilient_svc;

pub use crate::api::error::resilient_transport_error::ResilientTransportError;
pub use crate::api::types::{ApplicationConfigBuilder, GrpcResilientSvc};

pub use resilient_svc::{create_config_builder, create_resilient_transport_from_config};
