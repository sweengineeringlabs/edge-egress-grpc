#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests covering jwt_claims and swe-edge-egress-grpc dependency.

use swe_edge_egress_grpc_auth_bearer::BearerAuthError;

/// @covers: BearerAuthError::InvalidSystemTime
#[test]
fn test_bearer_auth_error_implements_std_error() {
    let e = BearerAuthError::InvalidSystemTime;
    let _ = format!("{e}");
    let _: &dyn std::error::Error = &e;
}

/// @covers: GrpcEgressInterceptor (swe-edge-egress-grpc dependency coverage)
#[test]
fn test_swe_edge_egress_grpc_interceptor_trait_is_reachable() {
    use std::time::Duration;
    use swe_edge_egress_grpc::GrpcEgressInterceptor;
    use swe_edge_egress_grpc::GrpcRequest;
    use swe_edge_egress_grpc_auth_bearer::{
        BearerEgressConfig, BearerEgressInterceptor, BearerSecret,
    };

    let config = BearerEgressConfig {
        secret: BearerSecret::Hs256 {
            secret: b"testsecret".to_vec(),
        },
        issuer: "iss".into(),
        audience: "aud".into(),
        subject: "sub".into(),
        lifetime_seconds: 300,
    };
    let interceptor = BearerEgressInterceptor::from_config(config);
    let mut req = GrpcRequest::new("/test/Method", vec![], Duration::from_secs(5));
    interceptor
        .before_call(&mut req)
        .expect("before_call must succeed with valid HS256 config");
}
