//! Coverage stub for `src/api/types/application_config_builder.rs`.

use swe_edge_egress_grpc_resilient::GrpcResilientSvc;

/// @covers: ApplicationConfigBuilder — create_config_builder returns a builder
#[test]
fn resilient_type_application_config_builder_is_accessible_int_test() {
    let builder = GrpcResilientSvc::create_config_builder();
    let _loader = builder.build_loader();
}
