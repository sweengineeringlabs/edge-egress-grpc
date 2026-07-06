//! Cheap PRNG for jitter — no `rand` crate dependency.
//!
//! SplitMix64 (one round per call): well-distributed, fast,
//! deterministically seedable for tests.  We only need 53 bits
//! of randomness to fill an `f64` mantissa, so quality beyond
//! that doesn't matter.  Not for cryptographic use.

use crate::api::{Error, NextUnitRequest, NextUnitResponse};

/// SplitMix64-based PRNG for jitter computation.
pub(crate) struct DefaultJitterRng {
    state: u64,
}

impl DefaultJitterRng {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            state: seed.wrapping_add(0x9E3779B97F4A7C15),
        }
    }

    /// Seed from the current wall clock — used as a default
    /// when the caller doesn't pin a seed for testing.
    pub(crate) fn from_clock() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| u64::from(d.subsec_nanos()) ^ d.as_secs().rotate_left(17))
            .unwrap_or(0);
        Self::new(nanos.wrapping_add(0xDEADBEEF))
    }

    /// Next uniform in `[0.0, 1.0)`.
    pub(crate) fn next_unit(&mut self) -> f64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^= z >> 31;
        ((z >> 11) as f64) * (1.0 / (1u64 << 53) as f64)
    }
}

impl crate::api::JitterRng for DefaultJitterRng {
    fn next_unit(&mut self, _req: NextUnitRequest) -> Result<NextUnitResponse, Error> {
        Ok(NextUnitResponse {
            value: DefaultJitterRng::next_unit(self),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_produces_value_in_unit_interval() {
        let mut rng = DefaultJitterRng::new(42);
        let v = rng.next_unit();
        assert!((0.0..1.0).contains(&v), "expected [0, 1), got {v}");
    }

    #[test]
    fn test_from_clock_produces_value_in_unit_interval() {
        let mut rng = DefaultJitterRng::from_clock();
        let v = rng.next_unit();
        assert!((0.0..1.0).contains(&v), "expected [0, 1), got {v}");
    }

    #[test]
    fn test_next_unit_is_deterministic_for_same_seed() {
        let v1 = DefaultJitterRng::new(99).next_unit();
        let v2 = DefaultJitterRng::new(99).next_unit();
        assert_eq!(v1, v2);
    }
}
