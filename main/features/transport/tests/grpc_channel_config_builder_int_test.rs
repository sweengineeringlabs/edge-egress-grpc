//! Integration tests for `GrpcChannelConfigBuilder`.

use std::time::Duration;
use swe_edge_egress_grpc_transport::{
    CompressionMode, GrpcChannelConfigBuilder, KeepAliveConfig, MtlsConfig, ResilienceConfig,
};

/// @covers: GrpcChannelConfigBuilder::build — valid endpoint returns Ok
#[test]
fn transport_struct_grpc_channel_config_builder_build_with_endpoint_returns_ok_int_test() {
    let cfg = GrpcChannelConfigBuilder::new()
        .endpoint("https://example.com:443")
        .build();
    assert!(cfg.is_ok());
}

/// @covers: GrpcChannelConfigBuilder::build — missing endpoint returns Err
#[test]
fn transport_struct_grpc_channel_config_builder_build_without_endpoint_returns_err_int_test() {
    assert!(GrpcChannelConfigBuilder::new().build().is_err());
}

/// @covers: GrpcChannelConfigBuilder::allow_plaintext — disables TLS
#[test]
fn transport_struct_grpc_channel_config_builder_allow_plaintext_disables_tls_int_test() {
    let cfg = GrpcChannelConfigBuilder::new()
        .endpoint("http://localhost:50051")
        .allow_plaintext()
        .build()
        .expect("build must succeed");
    assert!(!cfg.tls_required);
}

/// @covers: GrpcChannelConfigBuilder::mtls — stores mTLS identity
#[test]
fn transport_struct_grpc_channel_config_builder_mtls_stores_client_identity_int_test() {
    let m = MtlsConfig::new("cert.pem", "key.pem");
    let cfg = GrpcChannelConfigBuilder::new()
        .endpoint("https://example.com:443")
        .mtls(m)
        .build()
        .expect("build must succeed");
    assert!(cfg.mtls.is_some());
}

/// @covers: GrpcChannelConfigBuilder::keep_alive — stores config
#[test]
fn transport_struct_grpc_channel_config_builder_keep_alive_stores_config_int_test() {
    let ka = KeepAliveConfig {
        interval: Duration::from_secs(5),
        timeout: Duration::from_secs(10),
        permit_without_calls: true,
    };
    let cfg = GrpcChannelConfigBuilder::new()
        .endpoint("https://example.com:443")
        .keep_alive(ka)
        .build()
        .expect("build must succeed");
    assert!(cfg.keep_alive.is_some());
}

/// @covers: GrpcChannelConfigBuilder::max_message_bytes — overrides default
#[test]
fn transport_struct_grpc_channel_config_builder_max_message_bytes_overrides_default_int_test() {
    let cfg = GrpcChannelConfigBuilder::new()
        .endpoint("https://example.com:443")
        .max_message_bytes(8 * 1024 * 1024)
        .build()
        .expect("build must succeed");
    assert_eq!(cfg.max_message_bytes, 8 * 1024 * 1024);
}

/// @covers: GrpcChannelConfigBuilder::compression — sets mode
#[test]
fn transport_struct_grpc_channel_config_builder_compression_sets_mode_int_test() {
    let cfg = GrpcChannelConfigBuilder::new()
        .endpoint("https://example.com:443")
        .compression(CompressionMode::Gzip)
        .build()
        .expect("build must succeed");
    assert_eq!(cfg.compression, CompressionMode::Gzip);
}

/// @covers: GrpcChannelConfigBuilder::resilience — stores policy
#[test]
fn transport_struct_grpc_channel_config_builder_resilience_stores_policy_int_test() {
    let r = ResilienceConfig {
        max_attempts: 3,
        initial_backoff_ms: 100,
        backoff_multiplier: 2.0,
        jitter_factor: 0.1,
        max_backoff_ms: 2_000,
        rate_limit_max_attempts: 2,
        rate_limit_initial_backoff_ms: 1_000,
        rate_limit_max_backoff_ms: 10_000,
        failure_threshold: 5,
        cool_down_seconds: 10,
        half_open_probe_count: 1,
    };
    let cfg = GrpcChannelConfigBuilder::new()
        .endpoint("https://example.com:443")
        .resilience(r)
        .build()
        .expect("build must succeed");
    assert!(cfg.resilience.is_some());
}
