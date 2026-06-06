//! API layer — config, error, and contracts for the outbound bearer interceptor.

pub mod error;
pub mod traits;
pub mod types;
pub mod vo;

pub use error::BearerAuthError;
pub use traits::Validator;
pub use types::BearerEgressInterceptor;
pub use vo::{
    BearerEgressConfig, BearerEgressConfigBuilder, BearerSecret, JwtClaims, JwtClaimsBuilder,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
