//! Rule 125 API-level tests for `src/saf/resilient_svc.rs`.
//!
//! Covers the two public factory functions on `GrpcResilientSvc`:
//! `create_config_builder` and `create_resilient_transport_from_config`.

use swe_edge_egress_grpc::GrpcChannelConfig;
use swe_edge_egress_grpc_resilient::GrpcResilientSvc;

fn ensure_tls_provider() {
    use std::sync::Once;
    static ONCE: Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// @covers: GrpcResilientSvc::create_config_builder
#[test]
fn resilient_struct_grpc_resilient_svc_create_config_builder_returns_builder_int_test() {
    let builder = GrpcResilientSvc::create_config_builder();
    // build_loader must not panic.
    let _ = builder.build_loader();
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[test]
fn resilient_struct_grpc_resilient_svc_create_resilient_transport_returns_ok_int_test() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let result = GrpcResilientSvc::create_resilient_transport_from_config(&config);
    assert!(result.is_ok(), "expected Ok for valid plaintext config");
}
