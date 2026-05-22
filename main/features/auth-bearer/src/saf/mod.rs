//! SAF layer — public facade.

pub use crate::api::{
    BearerAuthError, BearerEgressConfig, BearerEgressInterceptor, BearerSecret,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
