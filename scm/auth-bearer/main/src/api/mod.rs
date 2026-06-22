//! API layer — config, error, and contracts for the outbound bearer interceptor.

pub(crate) mod error;
pub(crate) mod traits;
pub(crate) mod types;

pub use error::BearerAuthError;
pub use traits::Validator;
pub use types::BearerEgressInterceptor;
pub use types::{
    BearerEgressConfig, BearerEgressConfigBuilder, BearerSecret, JwtClaims, JwtClaimsBuilder,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
