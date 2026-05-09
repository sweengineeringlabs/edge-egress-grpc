//! SAF layer — public facade.

pub use crate::api::{
    BearerAuthError, BearerInboundConfig, BearerOutboundConfig, BearerSecret,
    BearerInboundInterceptor, BearerOutboundInterceptor,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
