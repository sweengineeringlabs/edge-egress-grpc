//! API layer — config, error, and contracts for the outbound bearer interceptor.

pub(crate) mod bearer;
pub mod error;
pub(crate) mod processor;
pub(crate) mod traits;
pub mod types;

pub use bearer::{
    BearerEgressConfig, BearerEgressInterceptor, BearerSecret, AUTHORIZATION_HEADER,
    EXTRACTED_BEARER_SUBJECT,
};
pub use error::BearerAuthError;
