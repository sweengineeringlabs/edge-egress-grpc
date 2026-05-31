//! Integration tests for the `Processor` trait contract in `swe-edge-egress-grpc-retry`.

use swe_edge_egress_grpc_retry::{GrpcRetryConfig, Processor};

struct AlwaysOk;

impl Processor for AlwaysOk {
    fn validate(&self, _config: &GrpcRetryConfig) -> Result<(), swe_edge_egress_grpc_retry::Error> {
        Ok(())
    }
}

/// @covers: Processor — trait is implementable
#[test]
fn grpc_retry_processor_custom_impl_accepts_config_int_test() {
    let p = AlwaysOk;
    let config = GrpcRetryConfig::default();
    assert!(p.validate(&config).is_ok());
}

/// @covers: Processor — trait is object-safe
#[test]
fn grpc_retry_processor_is_object_safe_int_test() {
    fn _assert(_: &dyn Processor) {}
}
