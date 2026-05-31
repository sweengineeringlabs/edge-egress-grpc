//! Integration tests for `api/status/conversions.rs`.

use swe_edge_egress_grpc_transport::Conversions;

#[test]
fn transport_struct_conversions_marker_is_constructable_int_test() {
    let _ = Conversions;
}
