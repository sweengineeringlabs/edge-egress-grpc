//! Composition site for [`Validator`] — one file per trait keeps wiring focused.

use crate::api::{ResilienceConfig, Validator};

/// Factory for the default [`Validator`].
pub struct ValidatorFactory;

impl ValidatorFactory {
    /// Construct the default [`Validator`] anchored on [`ResilienceConfig`].
    pub fn create() -> Box<dyn Validator> {
        Box::new(ResilienceConfig::default())
    }
}
