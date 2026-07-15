//! Local newtype wrapping the transport crate's `ResilienceConfigResilienceValidator`.

use edge_transport_grpc_egress::ResilienceConfigResilienceValidator as ForeignResilienceConfig;

/// Wraps `edge_transport_grpc_egress::ResilienceConfigResilienceValidator` so api/ never references
/// the foreign type directly — the delegating validation call lives in
/// `core/`. The field is `pub` (not `pub(crate)`) because callers must be
/// able to construct a [`crate::api::ConfigValidationRequest`] with real
/// policy values to exercise [`crate::api::Validator::validate`].
pub struct ResilienceConfig(pub ForeignResilienceConfig);
