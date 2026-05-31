//! Coverage stub for `src/api/factory/assembler.rs`.
//!
//! `Assembler` trait is `pub(crate)` — not part of the public API.
//! The concrete implementation is `GrpcResilientSvc::create_resilient_transport_from_config`.
//! This stub exercises that public factory which satisfies the `Assembler` contract.

use swe_edge_egress_grpc::GrpcChannelConfig;
use swe_edge_egress_grpc_resilient::GrpcResilientSvc;

fn ensure_tls_provider() {
    use std::sync::Once;
    static ONCE: Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// @covers: Assembler (internal) — GrpcResilientSvc implements assembly
#[test]
fn resilient_trait_assembler_create_transport_from_plaintext_config_int_test() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let result = GrpcResilientSvc::create_resilient_transport_from_config(&config);
    assert!(
        result.is_ok(),
        "assembly must succeed for valid plaintext config",
    );
}
