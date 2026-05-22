//! API layer — config, error, and constants for the outbound bearer interceptor.

pub(crate) mod application_config_builder;
pub(crate) mod architecture_config_builder;
pub(crate) mod bearer_auth_config;
pub(crate) mod bearer_auth_error;
pub(crate) mod bearer_egress_interceptor;
pub(crate) mod jwt_claims;
pub(crate) mod metadata_keys;

pub use bearer_auth_config::{BearerEgressConfig, BearerSecret};
pub use bearer_auth_error::BearerAuthError;
pub use bearer_egress_interceptor::BearerEgressInterceptor;
pub use metadata_keys::{AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT};
