//! Coverage stub for `src/api/error/resilient_transport_error.rs`.

use edge_transport_grpc_egress::GrpcChannelConfigError;
use edge_transport_grpc_egress_resilient::ResilientTransportError;

/// @covers: ResilientTransportError::ChannelConfig — wraps and displays the source error
#[test]
fn resilient_enum_resilient_transport_error_is_accessible_int_test() {
    let err: ResilientTransportError =
        GrpcChannelConfigError::PlaintextRejected("http://x".into()).into();
    let s = err.to_string();
    assert!(
        s.contains("http://x"),
        "ChannelConfig must transparently forward the source error's display, got: {s}",
    );
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
