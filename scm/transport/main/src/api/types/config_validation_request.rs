//! `ConfigValidationRequest` — request for [`crate::api::ResilienceValidator::validate_config`].

use crate::api::types::resilience_config::ResilienceConfig;

/// Request carrying the [`ResilienceConfig`] to validate.
#[derive(Debug, Clone)]
pub struct ConfigValidationRequest {
    /// The configuration under validation.
    pub config: ResilienceConfig,
}
