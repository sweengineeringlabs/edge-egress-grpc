//! Value objects — bearer-auth configuration, secret material, JWT claims,
//! and reserved metadata keys.

pub(crate) mod bearer_egress_config;
pub(crate) mod bearer_egress_config_builder;
pub(crate) mod bearer_secret;
pub(crate) mod jwt_claims;
pub(crate) mod jwt_claims_builder;
pub(crate) mod metadata_keys;

pub use bearer_egress_config::BearerEgressConfig;
pub use bearer_egress_config_builder::BearerEgressConfigBuilder;
pub use bearer_secret::BearerSecret;
pub use jwt_claims::JwtClaims;
pub use jwt_claims_builder::JwtClaimsBuilder;
pub use metadata_keys::{AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT};
