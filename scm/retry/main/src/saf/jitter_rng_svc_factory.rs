//! Composition site for [`JitterRng`] — one file per trait keeps wiring focused.

use crate::api::JitterRng;
use crate::core::retry::traits::jitter_rng::DefaultJitterRng;

/// Factory for the default [`JitterRng`].
pub struct JitterRngFactory;

impl JitterRngFactory {
    /// Construct the default [`JitterRng`], seeded from the wall clock.
    pub fn create() -> Box<dyn JitterRng> {
        Box::new(DefaultJitterRng::from_clock())
    }
}
