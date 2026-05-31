//! Integration tests for `KeepAliveConfig`.

use std::time::Duration;
use swe_edge_egress_grpc_transport::KeepAliveConfig;

/// @covers: KeepAliveConfig::default — uses recommended gRPC keep-alive intervals
#[test]
fn transport_struct_keep_alive_config_default_uses_recommended_grpc_intervals_int_test() {
    let cfg = KeepAliveConfig::default();
    assert_eq!(cfg.interval, Duration::from_secs(10));
    assert_eq!(cfg.timeout, Duration::from_secs(20));
    assert!(!cfg.permit_without_calls);
}
