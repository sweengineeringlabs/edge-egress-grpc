//! Integration tests for the `Processor` trait (`api/traits.rs`).
//!
//! Verifies that `TonicGrpcClient` — the primary `Processor` implementor —
//! satisfies the `describe_processor` SAF contract.

use swe_edge_egress_grpc_transport::{create_transport_from_config, GrpcChannelConfig};

fn ensure_rustls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// @covers: describe_processor
#[test]
fn transport_struct_processor_describe_returns_non_empty_label_int_test() {
    ensure_rustls_provider();
    let client = swe_edge_egress_grpc_transport::TonicGrpcClient::new("http://127.0.0.1:50051");
    let label = swe_edge_egress_grpc_transport::describe_processor(&client);
    assert!(
        !label.is_empty(),
        "describe_processor must return a non-empty label, got: {label:?}"
    );
}

/// @covers: describe_processor
#[test]
fn transport_struct_processor_describe_returns_tonic_grpc_client_label_int_test() {
    ensure_rustls_provider();
    let client = swe_edge_egress_grpc_transport::TonicGrpcClient::new("http://127.0.0.1:50051");
    let label = swe_edge_egress_grpc_transport::describe_processor(&client);
    assert_eq!(
        label, "tonic-grpc-client",
        "TonicGrpcClient label must be 'tonic-grpc-client', got: {label:?}"
    );
}

/// Verify the `Processor` trait object is accessible and the SAF factory
/// returns a transport that can be used in a processor context.
#[test]
fn transport_struct_processor_factory_creates_valid_transport_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let transport = create_transport_from_config(&cfg).expect("create transport");
    // If transport is created, the Processor impl is wired correctly.
    let _ = transport;
}
