//! Coverage stub for `src/api/breaker/error.rs`.

use edge_transport_grpc_egress_breaker::Error;

/// @covers: Error::ParseFailed — display includes crate name
#[test]
fn breaker_enum_error_parse_failed_display_includes_crate_name_int_test() {
    let err = Error::ParseFailed("missing field `failure_threshold`".into());
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_grpc_egress_breaker"),
        "expected crate name in display, got: {s}",
    );
}

/// @covers: Error::InvalidConfig — display includes crate name
#[test]
fn breaker_enum_error_invalid_config_display_includes_crate_name_int_test() {
    let err = Error::InvalidConfig("failure_threshold must be >= 1".into());
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_grpc_egress_breaker"),
        "expected crate name in display, got: {s}",
    );
}
