//! Rule 125 API-level tests for `src/saf/resilient_svc.rs`.
//!
//! Covers the two public factory functions on `GrpcResilientSvc`:
//! `create_config_builder` and `create_resilient_transport_from_config`.

#![allow(clippy::unwrap_used, clippy::expect_used)]
use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgressError};
use swe_edge_egress_grpc_resilient::GrpcResilientSvc;

fn ensure_tls_provider() {
    use std::sync::Once;
    static ONCE: Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

#[derive(serde::Deserialize, Default, PartialEq, Debug)]
struct AbsentSectionProbe {
    marker: bool,
}

/// @covers: GrpcResilientSvc::create_config_builder
#[test]
fn resilient_struct_grpc_resilient_svc_create_config_builder_returns_builder_int_test() {
    let loader = GrpcResilientSvc::create_config_builder()
        .build_loader()
        .expect("a builder pre-seeded with name and version must build a valid loader");
    // In a test environment there is no application.toml at any configured
    // directory, so querying any section must fail with NotFound — proves
    // the loader is genuinely wired to the filesystem, not a no-op stub.
    let err = loader
        .load_section::<AbsentSectionProbe>("resilient_test_probe_section_that_does_not_exist")
        .expect_err("no config directory exists in the test environment");
    assert!(
        err.to_string()
            .contains("resilient_test_probe_section_that_does_not_exist"),
        "error must name the missing section, got: {err}"
    );
}

/// @covers: GrpcResilientSvc::create_resilient_transport_from_config
#[tokio::test]
async fn resilient_struct_grpc_resilient_svc_create_resilient_transport_returns_ok_int_test() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let transport = GrpcResilientSvc::create_resilient_transport_from_config(&config)
        .expect("expected Ok for valid plaintext config");
    let health = transport.health_check().await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}
