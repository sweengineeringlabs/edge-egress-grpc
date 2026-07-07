//! Composition site for [`Validator`] — one file per trait keeps wiring focused.

use crate::api::{ResilienceConfigResilienceValidator, Validator};

/// Factory for the default [`Validator`].
pub struct ValidatorFactory;

impl ValidatorFactory {
    /// Construct the default [`Validator`] anchored on [`ResilienceConfigResilienceValidator`].
    pub fn create() -> Box<dyn Validator> {
        Box::new(ResilienceConfigResilienceValidator::default())
    }
}
