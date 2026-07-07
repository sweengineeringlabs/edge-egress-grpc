//! `ConfigValidationRequest` — request for [`crate::api::ResilienceValidator::validate_config`].

use std::sync::Arc;

use crate::api::Validator;

/// Request carrying the config to validate as an opaque [`Validator`] trait
/// object — per `field_type_purity`, api/ struct fields must be trait objects
/// or value types, never a concrete config struct by value. The receiver is
/// validated purely through [`Validator::validate`], never by reading its
/// concrete fields.
#[derive(Clone)]
pub struct ConfigValidationRequest {
    /// The configuration under validation.
    pub config: Arc<dyn Validator>,
}
