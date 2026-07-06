//! Interface counterpart for the corresponding core/ implementation.

use crate::api::Error;
use crate::api::NextUnitRequest;
use crate::api::NextUnitResponse;

/// Trait for jitter RNG implementations used in backoff computation.
pub trait JitterRng: Send + Sync {
    /// Return the next uniform sample in `[0.0, 1.0)`.
    fn next_unit(&mut self, req: NextUnitRequest) -> Result<NextUnitResponse, Error>;
}
