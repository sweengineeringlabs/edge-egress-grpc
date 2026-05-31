//! Integration tests for `TonicGrpcClientBuilder`.
//!
//! Tests that access internal `pub(crate)` fields cannot be ported and are
//! omitted. Only tests using the public API surface are included here.

use swe_edge_egress_grpc_transport::{
    CompressionMode, GrpcEgressInterceptorChain, TonicGrpcClientBuilder,
};

fn ensure_rustls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// @covers: TonicGrpcClientBuilder::new, build — builder produces a client without panic
#[test]
fn transport_struct_tonic_grpc_client_builder_build_produces_client_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://localhost:50051").build();
    let _ = std::mem::size_of_val(&client);
}

/// @covers: TonicGrpcClientBuilder::timeout — fluent setter returns Self
#[test]
fn transport_struct_tonic_grpc_client_builder_timeout_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://localhost:50051")
        .timeout(std::time::Duration::from_secs(5))
        .build();
    let _ = std::mem::size_of_val(&client);
}

/// @covers: TonicGrpcClientBuilder::max_message_bytes — fluent setter returns Self
#[test]
fn transport_struct_tonic_grpc_client_builder_max_message_bytes_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://localhost:50051")
        .max_message_bytes(8 * 1024 * 1024)
        .build();
    let _ = std::mem::size_of_val(&client);
}

/// @covers: TonicGrpcClientBuilder::compression — fluent setter returns Self
#[test]
fn transport_struct_tonic_grpc_client_builder_compression_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let client = TonicGrpcClientBuilder::new("http://localhost:50051")
        .compression(CompressionMode::Gzip)
        .build();
    let _ = std::mem::size_of_val(&client);
}

/// @covers: TonicGrpcClientBuilder::interceptors — fluent setter returns Self
#[test]
fn transport_struct_tonic_grpc_client_builder_interceptors_setter_is_fluent_int_test() {
    ensure_rustls_provider();
    let chain = GrpcEgressInterceptorChain::new();
    let client = TonicGrpcClientBuilder::new("http://localhost:50051")
        .interceptors(chain)
        .build();
    let _ = std::mem::size_of_val(&client);
}
