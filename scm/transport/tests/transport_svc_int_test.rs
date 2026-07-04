//! Coverage stub for `api/types/transport_svc.rs`.

use swe_edge_egress_grpc_transport::TransportSvc;

/// @covers: TransportSvc struct is publicly accessible and is a zero-sized namespace marker
#[test]
fn transport_struct_transport_svc_exists_int_test() {
    assert_eq!(
        std::mem::size_of::<TransportSvc>(),
        0,
        "TransportSvc is a namespace marker for factory fns and must carry no instance state"
    );
}
