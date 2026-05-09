//! API layer — config, error, reserved-key constants.

pub(crate) mod bearer_auth_config;
pub(crate) mod bearer_auth_error;
pub(crate) mod bearer_inbound_interceptor;
pub(crate) mod bearer_outbound_interceptor;
pub(crate) mod jwt_claims;
pub(crate) mod metadata_keys;

pub use bearer_auth_config::{BearerInboundConfig, BearerOutboundConfig, BearerSecret};
pub use bearer_auth_error::BearerAuthError;
pub use bearer_inbound_interceptor::BearerInboundInterceptor;
pub use bearer_outbound_interceptor::BearerOutboundInterceptor;
pub use metadata_keys::{AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT};
