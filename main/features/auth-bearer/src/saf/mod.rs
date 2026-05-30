//! SAF layer — public facade.

mod bearer_svc;

pub use crate::api::types::GrpcAuthBearerSvc;

pub use crate::api::{
    BearerAuthError, BearerEgressConfig, BearerEgressInterceptor, BearerSecret,
    AUTHORIZATION_HEADER, EXTRACTED_BEARER_SUBJECT,
};
