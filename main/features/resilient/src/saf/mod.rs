//! SAF layer — public facade.

mod factory;

pub use crate::api::error::ResilientTransportError;
pub use factory::create_resilient_transport_from_config;
