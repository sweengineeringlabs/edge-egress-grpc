//! Outbound interceptor: signs a JWT and injects it as
//! `authorization: Bearer <jwt>` on every request.

use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use swe_edge_egress_grpc::{
    GrpcEgressError, GrpcEgressInterceptor, GrpcRequest, GrpcResponse, GrpcStatusCode,
};

use crate::api::bearer::bearer_secret::BearerSecret;
use crate::api::traits::Processor;
use crate::api::{BearerAuthError, BearerEgressInterceptor, AUTHORIZATION_HEADER};
use crate::core::bearer::jwt_claims::JwtClaims;

impl Processor for BearerEgressInterceptor {}

impl BearerEgressInterceptor {
    fn sign_token(&self) -> Result<String, BearerAuthError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| BearerAuthError::InvalidSystemTime)?
            .as_secs();
        let claims = JwtClaims {
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            sub: self.config.subject.clone(),
            iat: now,
            exp: now.saturating_add(self.config.lifetime_seconds),
        };
        let (alg, key) = match &self.config.secret {
            BearerSecret::Hs256 { secret } => (Algorithm::HS256, EncodingKey::from_secret(secret)),
            BearerSecret::Rs256 { private_pem } => (
                Algorithm::RS256,
                EncodingKey::from_rsa_pem(private_pem).map_err(BearerAuthError::SignFailed)?,
            ),
        };
        jsonwebtoken::encode(&Header::new(alg), &claims, &key).map_err(BearerAuthError::SignFailed)
    }
}

impl GrpcEgressInterceptor for BearerEgressInterceptor {
    fn before_call(&self, req: &mut GrpcRequest) -> Result<(), GrpcEgressError> {
        let token = self.sign_token().map_err(|e| {
            tracing::warn!(error = %e, "failed to mint bearer token");
            GrpcEgressError::Status(
                GrpcStatusCode::Internal,
                "failed to mint bearer token".into(),
            )
        })?;
        req.metadata
            .headers
            .insert(AUTHORIZATION_HEADER.to_string(), format!("Bearer {token}"));
        Ok(())
    }

    fn after_call(&self, _resp: &mut GrpcResponse) -> Result<(), GrpcEgressError> {
        Ok(())
    }
}
