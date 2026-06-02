//! SAF layer — public facade.

mod bearer_svc;

pub use crate::api::types::GrpcAuthBearerSvc;

pub use crate::api::{
    BearerAuthError, BearerEgressConfig, BearerEgressConfigBuilder, BearerEgressInterceptor,
    BearerSecret, JwtClaims, JwtClaimsBuilder, Validator, AUTHORIZATION_HEADER,
    EXTRACTED_BEARER_SUBJECT,
};
