//! Bearer auth API — grouped types and constants.

pub(crate) mod bearer_egress_config;
pub(crate) mod bearer_egress_interceptor;
pub(crate) mod bearer_secret;
pub(crate) mod jwt_claims;
pub(crate) mod metadata_keys;

pub use bearer_egress_config::BearerEgressConfig;
pub use bearer_egress_interceptor::BearerEgressInterceptor;
pub use bearer_secret::BearerSecret;
pub use jwt_claims::JwtClaims;
pub use metadata_keys::{AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT};
