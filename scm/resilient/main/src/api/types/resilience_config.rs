//! Local newtype wrapping the transport crate's `ResilienceConfigResilienceValidator`.

use swe_edge_egress_grpc::ResilienceConfigResilienceValidator as ForeignResilienceConfig;

/// Wraps `swe_edge_egress_grpc::ResilienceConfigResilienceValidator` so api/ never references
/// the foreign type directly — the delegating validation call lives in
/// `core/`. The field is `pub` (not `pub(crate)`) because callers must be
/// able to construct a [`crate::api::ConfigValidationRequest`] with real
/// policy values to exercise [`crate::api::Validator::validate`].
pub struct ResilienceConfig(pub ForeignResilienceConfig);
