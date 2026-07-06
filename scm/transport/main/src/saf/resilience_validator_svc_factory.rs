//! Composition site for [`ResilienceValidator`] — one file per trait keeps wiring focused.

use crate::api::{ResilienceConfig, ResilienceValidator};

/// Factory for the default [`ResilienceValidator`].
pub struct ResilienceValidatorFactory;

impl ResilienceValidatorFactory {
    /// Construct the default [`ResilienceValidator`] anchored on [`ResilienceConfig`].
    pub fn create() -> Box<dyn ResilienceValidator> {
        Box::new(ResilienceConfig::default())
    }
}
