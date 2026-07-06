//! Local newtype wrapping the transport crate's `ResilienceConfig`.

use swe_edge_egress_grpc::ResilienceConfig as ForeignResilienceConfig;

/// Wraps `swe_edge_egress_grpc::ResilienceConfig` so api/ never references
/// the foreign type directly — the delegating validation call lives in
/// `core/`. The field is `pub` (not `pub(crate)`) because callers must be
/// able to construct a [`crate::api::ConfigValidationRequest`] with real
/// policy values to exercise [`crate::api::Validator::validate`].
pub struct ResilienceConfig(pub ForeignResilienceConfig);
