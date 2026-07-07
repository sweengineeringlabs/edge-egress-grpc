#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests verifying the concrete transport client satisfies the
//! `Processor` contract through the `TransportSvc` SAF factory.

use swe_edge_egress_grpc_transport::{GrpcChannelConfig, TransportSvc};

fn ensure_rustls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// @covers: TransportSvc::create_transport_from_config — factory produces a valid transport
#[test]
fn transport_struct_processor_factory_creates_valid_transport_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let transport = TransportSvc::create_transport_from_config(&cfg).expect("create transport");
    let _ = transport;
}

/// @covers: TransportSvc::create_tonic_client_from_config — returns concrete client
#[test]
fn transport_struct_processor_tonic_client_created_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let client = TransportSvc::create_tonic_client_from_config(&cfg).expect("tonic client");
    let _ = client;
}
