//! Coverage stub for `src/api/types/application_config_builder.rs`.

use swe_edge_egress_grpc_retry::GrpcRetrySvc;

/// @covers: ApplicationConfigBuilder — create_config_builder returns a builder
#[test]
fn retry_type_application_config_builder_is_accessible_int_test() {
    let builder = GrpcRetrySvc::create_config_builder();
    let _loader = builder.build_loader();
}
