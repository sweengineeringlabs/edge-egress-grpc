#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for [`BearerEgressInterceptor`].

use std::time::Duration;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use swe_edge_egress_grpc::GrpcRequest;
use swe_edge_egress_grpc_auth_bearer::{
    BearerEgressConfig, BearerEgressInterceptor, BearerSecret, AUTHORIZATION_HEADER,
};

fn hs256_config(secret: &[u8]) -> BearerEgressConfig {
    BearerEgressConfig {
        secret: BearerSecret::Hs256 {
            secret: secret.to_vec(),
        },
        issuer: "test-iss".into(),
        audience: "test-aud".into(),
        subject: "test-sub".into(),
        lifetime_seconds: 60,
    }
}

/// @covers: BearerEgressInterceptor::from_config
#[test]
fn test_from_config_constructs_interceptor() {
    let _ = BearerEgressInterceptor::from_config(hs256_config(b"sec"));
}

/// @covers: BearerEgressInterceptor::before_call
#[test]
fn test_before_call_injects_bearer_authorization_header() {
    use swe_edge_egress_grpc::GrpcEgressInterceptor;

    let interceptor = BearerEgressInterceptor::from_config(hs256_config(b"sec"));
    let mut req = GrpcRequest::new("/svc/M", vec![], Duration::from_secs(1));
    interceptor.before_call(&mut req).expect("before_call");
    let auth = req
        .metadata
        .headers
        .get(AUTHORIZATION_HEADER)
        .cloned()
        .expect("authorization header must be injected");
    assert!(
        auth.starts_with("Bearer "),
        "expected Bearer-prefixed header, got {auth}"
    );
    let token = auth.trim_start_matches("Bearer ");
    assert_eq!(
        token.matches('.').count(),
        2,
        "JWT must have 3 segments: {token}"
    );
}

/// @covers: BearerEgressInterceptor::sign_token (via before_call round-trip with swe-edge-configbuilder and jsonwebtoken)
#[test]
fn test_sign_token_round_trips_through_jsonwebtoken_verifier() {
    use serde::{Deserialize, Serialize};
    use swe_edge_egress_grpc::GrpcEgressInterceptor;

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        iss: String,
        aud: String,
        sub: String,
        exp: u64,
        iat: u64,
    }

    let interceptor = BearerEgressInterceptor::from_config(hs256_config(b"sec"));
    let mut req = GrpcRequest::new("/svc/M", vec![], Duration::from_secs(1));
    interceptor.before_call(&mut req).expect("before_call");
    let auth = req
        .metadata
        .headers
        .get(AUTHORIZATION_HEADER)
        .cloned()
        .expect("auth header");
    let token = auth.trim_start_matches("Bearer ");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["test-aud"]);
    validation.set_issuer(&["test-iss"]);
    let decoded = decode::<Claims>(token, &DecodingKey::from_secret(b"sec"), &validation)
        .expect("token must be verifiable with matching key");
    assert_eq!(decoded.claims.iss, "test-iss");
    assert_eq!(decoded.claims.aud, "test-aud");
    assert_eq!(decoded.claims.sub, "test-sub");
}

/// @covers: subtle constant-time comparison via BearerSecret::ct_eq_hs256
#[test]
fn test_subtle_constant_time_eq_used_for_hs256_comparison() {
    let a = BearerSecret::Hs256 {
        secret: b"same-secret".to_vec(),
    };
    let b = BearerSecret::Hs256 {
        secret: b"same-secret".to_vec(),
    };
    assert!(
        a.ct_eq_hs256(&b),
        "ct_eq_hs256 must return true for equal HS256 secrets"
    );
}

/// @covers: swe-edge-configbuilder integration
#[test]
fn test_config_builder_can_be_created_via_saf() {
    use swe_edge_egress_grpc_auth_bearer::GrpcAuthBearerSvc;
    let builder = GrpcAuthBearerSvc::create_config_builder();
    // The builder is properly seeded — just verify it constructs without panic.
    let _ = builder;
}
