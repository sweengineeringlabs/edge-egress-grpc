//! Coverage stub for `src/api/types/application_config_builder.rs`.

use swe_edge_egress_grpc_breaker::GrpcBreakerSvc;

/// @covers: ApplicationConfigBuilder — create_config_builder returns a builder
#[test]
fn breaker_type_application_config_builder_is_accessible_int_test() {
    let builder = GrpcBreakerSvc::create_config_builder();
    // The builder is usable — build_loader produces a loader without panicking.
    let _loader = builder.build_loader();
}
