//! SAF layer — public facade.

pub use crate::api::{
    BearerAuthError, BearerOutboundConfig, BearerOutboundInterceptor, BearerSecret,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
