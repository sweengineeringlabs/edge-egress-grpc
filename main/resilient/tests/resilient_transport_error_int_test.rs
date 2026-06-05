//! Coverage stub for `src/api/error/resilient_transport_error.rs`.

use swe_edge_egress_grpc_resilient::ResilientTransportError;

/// @covers: ResilientTransportError — type is accessible
#[test]
fn resilient_enum_resilient_transport_error_is_accessible_int_test() {
    let _ = std::mem::size_of::<ResilientTransportError>();
}

/// @covers: ResilientTransportError::InvalidResilience — display is non-empty
#[test]
fn resilient_enum_resilient_transport_error_invalid_resilience_display_is_non_empty_int_test() {
    let err = ResilientTransportError::InvalidResilience("max_attempts must be >= 1".into());
    let s = err.to_string();
    assert!(
        s.contains("invalid resilience config"),
        "expected 'invalid resilience config' in display, got: {s}",
    );
}
