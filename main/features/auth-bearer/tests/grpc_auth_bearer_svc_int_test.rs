//! Integration tests for [`GrpcAuthBearerSvc`].

use swe_edge_egress_grpc_auth_bearer::GrpcAuthBearerSvc;

/// @covers: GrpcAuthBearerSvc::create_config_builder
#[test]
fn test_create_config_builder_returns_seeded_builder() {
    let builder = GrpcAuthBearerSvc::create_config_builder();
    // The builder holds the crate name and version — just verify no panic.
    let _ = builder;
}
