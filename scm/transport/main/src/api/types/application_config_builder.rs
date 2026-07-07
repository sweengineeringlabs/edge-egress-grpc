//! Config builder for `swe_edge_egress_grpc_transport`.

use swe_edge_configbuilder::ConfigBuilderImpl;

/// Wraps the external `swe_edge_configbuilder::ConfigBuilderImpl` so api/
/// never references a foreign crate type directly — conversions and the
/// delegating `build_loader` method live in `core/`.
pub struct ApplicationConfigBuilder(pub(crate) ConfigBuilderImpl);
