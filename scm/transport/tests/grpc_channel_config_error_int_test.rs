//! Integration tests for `api/port/grpc/grpc_channel_config_error.rs`.

use edge_transport_grpc_egress_transport::GrpcChannelConfigError;

#[test]
fn transport_struct_plaintext_rejected_display_includes_endpoint_int_test() {
    let e = GrpcChannelConfigError::PlaintextRejected("http://x".into());
    assert!(e.to_string().contains("http://x"));
}

#[test]
fn transport_struct_config_display_includes_reason_int_test() {
    let e = GrpcChannelConfigError::Config("max_attempts must be >= 1".into());
    assert!(e.to_string().contains("max_attempts"));
}
