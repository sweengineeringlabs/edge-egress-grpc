//! API layer — config, error, and contracts for the outbound bearer interceptor.

pub(crate) mod bearer;
pub mod error;
pub mod traits;
pub mod types;

pub use bearer::{
    BearerEgressConfig, BearerEgressConfigBuilder, BearerEgressInterceptor, BearerSecret,
    JwtClaims, JwtClaimsBuilder, AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
pub use error::BearerAuthError;
pub use traits::{Processor, Validator};
