//! Integration tests for `api/value/grpc/grpc_metadata.rs`.

use swe_edge_egress_grpc_transport::GrpcMetadata;

#[test]
fn transport_struct_grpc_metadata_default_has_empty_headers_int_test() {
    let m = GrpcMetadata::default();
    assert!(m.headers.is_empty());
}

#[test]
fn transport_struct_with_header_inserts_entry_into_headers_map_int_test() {
    let m = GrpcMetadata::default().with_header("x-request-id", "req-1");
    assert_eq!(
        m.headers.get("x-request-id").map(String::as_str),
        Some("req-1")
    );
}

#[test]
fn transport_struct_with_header_chaining_accumulates_entries_int_test() {
    let m = GrpcMetadata::default()
        .with_header("a", "1")
        .with_header("b", "2");
    assert_eq!(m.headers.len(), 2);
}
