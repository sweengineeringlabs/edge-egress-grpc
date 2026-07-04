//! Request for [`crate::api::traits::Validator::validate`].

use crate::api::types::grpc_breaker_config::GrpcBreakerConfig;

/// Input to [`crate::api::traits::Validator::validate`] — the config to check.
pub struct ConfigValidationRequest {
    /// The breaker policy to validate.
    pub config: GrpcBreakerConfig,
}
