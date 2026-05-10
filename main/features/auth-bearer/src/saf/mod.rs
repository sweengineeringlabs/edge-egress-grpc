//! SAF layer — public facade.

pub use crate::api::{
    BearerAuthError, BearerInboundConfig, BearerInboundInterceptor, BearerOutboundConfig,
    BearerOutboundInterceptor, BearerSecret, AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
