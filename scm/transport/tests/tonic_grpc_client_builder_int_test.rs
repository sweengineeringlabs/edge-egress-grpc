//! Integration tests for `TonicGrpcClientBuilder`.
//!
//! Tests that access internal `pub(crate)` fields cannot be ported and are
//! omitted (field-level verification lives in the crate's inline tests).
//! These exercise the public API surface only: each fluent path must
//! produce a genuinely connectable `GrpcEgress`, not merely compile.

use swe_edge_egress_grpc_transport::{
    CompressionMode, GrpcEgress, GrpcEgressError, GrpcEgressInterceptorChain,
    TonicGrpcClientBuilder,
};

fn ensure_rustls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// Nothing listens on 127.0.0.1:50051 in the test environment, so a real
/// health_check() call must genuinely fail — proves the built client is a
/// real, connectable GrpcEgress wired to the given base_uri, not a stub.
async fn assert_genuinely_connectable(client: impl GrpcEgress) {
    let health = client.health_check().await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}

/// @covers: TonicGrpcClientBuilder::new, build — builder produces a client without panic
#[tokio::test]
async fn transport_struct_tonic_grpc_client_builder_build_produces_client_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://127.0.0.1:50051").build();
    assert_genuinely_connectable(client).await;
}

/// @covers: TonicGrpcClientBuilder::timeout — fluent setter returns Self
#[tokio::test]
async fn transport_struct_tonic_grpc_client_builder_timeout_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://127.0.0.1:50051")
        .timeout(std::time::Duration::from_secs(5))
        .build();
    assert_genuinely_connectable(client).await;
}

/// @covers: TonicGrpcClientBuilder::max_message_bytes — fluent setter returns Self
#[tokio::test]
async fn transport_struct_tonic_grpc_client_builder_max_message_bytes_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://127.0.0.1:50051")
        .max_message_bytes(8 * 1024 * 1024)
        .build();
    assert_genuinely_connectable(client).await;
}

/// @covers: TonicGrpcClientBuilder::compression — fluent setter returns Self
#[tokio::test]
async fn transport_struct_tonic_grpc_client_builder_compression_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://127.0.0.1:50051")
        .compression(CompressionMode::Gzip)
        .build();
    assert_genuinely_connectable(client).await;
}

/// @covers: TonicGrpcClientBuilder::interceptors — fluent setter returns Self
#[tokio::test]
async fn transport_struct_tonic_grpc_client_builder_interceptors_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let chain = GrpcEgressInterceptorChain::new();
    let client = TonicGrpcClientBuilder::new("http://127.0.0.1:50051")
        .interceptors(chain)
        .build();
    assert_genuinely_connectable(client).await;
}
