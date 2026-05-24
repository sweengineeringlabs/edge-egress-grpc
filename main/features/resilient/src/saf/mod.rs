//! SAF layer — public facade.

mod factory;

pub use crate::api::error::ResilientTransportError;
pub use factory::{create_config_builder, create_resilient_transport_from_config};
