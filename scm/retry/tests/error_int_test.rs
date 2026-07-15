//! Integration tests for [`Error`] — the crate's domain error type.

use edge_transport_grpc_egress_retry::Error;

/// @covers: ParseFailed
#[test]
fn test_parse_failed_display_names_crate_and_reason() {
    let err = Error::ParseFailed("missing field `max_attempts`".into());
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_grpc_egress_retry"),
        "missing crate name: {s}"
    );
    assert!(s.contains("max_attempts"), "missing field name: {s}");
}

/// @covers: InvalidConfig
#[test]
fn test_invalid_config_display_includes_crate_name() {
    let err = Error::InvalidConfig("backoff_multiplier must be > 0".into());
    let s = err.to_string();
    assert!(
        s.contains("edge_transport_grpc_egress_retry"),
        "missing crate name: {s}"
    );
    assert!(s.contains("backoff_multiplier"), "missing field name: {s}");
}
