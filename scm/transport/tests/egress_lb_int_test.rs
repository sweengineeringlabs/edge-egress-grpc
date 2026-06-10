//! Integration tests for the load-balanced gRPC egress adapter.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use swe_edge_egress_grpc_transport::{
    BackendConfig, LoadbalancerConfig, Strategy, TonicLbGrpcClient, TransportSvc,
};

fn one_backend_config(url: &str) -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: url.to_string(),
            weight: 1,
        }],
    }
}

// ── constructor: error paths ─────────────────────────────────────────────────
// These fail before touching tonic's runtime (empty-backends / bad URL checks),
// so they are safe as plain sync tests.

#[test]
fn test_from_config_empty_backends_returns_unavailable() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![],
    };
    let err = TonicLbGrpcClient::from_config(config).unwrap_err();
    assert!(
        err.to_string().contains("no backends"),
        "expected 'no backends' in '{err}'"
    );
}

#[test]
fn test_from_config_invalid_url_returns_unavailable() {
    let config = LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![BackendConfig {
            url: "not a valid uri !!!".to_string(),
            weight: 1,
        }],
    };
    let err = TonicLbGrpcClient::from_config(config).unwrap_err();
    assert!(
        err.to_string().contains("invalid backend URL"),
        "expected 'invalid backend URL' in '{err}'"
    );
}

// ── constructor: happy path ───────────────────────────────────────────────────
// Channel::balance_list requires a Tokio runtime → async tests.

#[tokio::test]
async fn test_from_config_valid_url_builds_successfully() {
    let config = one_backend_config("http://localhost:50051");
    assert!(TonicLbGrpcClient::from_config(config).is_ok());
}

#[tokio::test]
async fn test_with_timeout_overrides_default() {
    let config = one_backend_config("http://localhost:50051");
    let client = TonicLbGrpcClient::from_config(config)
        .unwrap()
        .with_timeout(Duration::from_secs(5));
    assert_eq!(client.timeout(), Duration::from_secs(5));
}

// ── SAF factory ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_lb_transport_from_config_returns_dyn_egress() {
    let config = one_backend_config("http://localhost:50051");
    let transport = TransportSvc::create_lb_transport_from_config(config);
    assert!(transport.is_ok());
}

// ── object safety ────────────────────────────────────────────────────────────

#[test]
fn test_grpc_egress_trait_is_object_safe() {
    use swe_edge_egress_grpc_transport::GrpcEgress;
    fn _assert(_: &dyn GrpcEgress) {}
}

// ── health check ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_health_check_with_healthy_pool_returns_ok() {
    use swe_edge_egress_grpc_transport::GrpcEgress;
    let config = one_backend_config("http://localhost:50051");
    let client = TonicLbGrpcClient::from_config(config).unwrap();
    // Pool has one healthy backend — health_check only probes pool membership,
    // no network call is made.
    assert!(client.health_check().await.is_ok());
}
