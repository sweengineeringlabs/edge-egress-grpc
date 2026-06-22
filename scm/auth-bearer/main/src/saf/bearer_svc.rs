//! gRPC auth-bearer SAF — factory methods on [`GrpcAuthBearerSvc`].

pub use crate::api::types::grpc_auth_bearer_svc::GrpcAuthBearerSvc;

pub use crate::api::{
    BearerAuthError, BearerEgressConfig, BearerEgressConfigBuilder, BearerEgressInterceptor,
    BearerSecret, JwtClaims, JwtClaimsBuilder, Validator, AUTHORIZATION_HEADER,
    EXTRACTED_BEARER_SUBJECT,
};

impl GrpcAuthBearerSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }
}
