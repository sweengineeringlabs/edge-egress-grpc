//! Composition site for [`BreakerTransition`] — one file per trait keeps
//! wiring focused.

use crate::api::BreakerTransition;
use crate::core::breaker::breaker_transition::DefaultBreakerTransition;

/// Factory for the default [`BreakerTransition`].
pub struct BreakerTransitionFactory;

impl BreakerTransitionFactory {
    /// Construct the default [`BreakerTransition`].
    pub fn create() -> Box<dyn BreakerTransition> {
        Box::new(DefaultBreakerTransition)
    }
}
