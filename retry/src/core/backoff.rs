//! Exponential-backoff-with-jitter scheduler.
//!
//! Pure logic — given the policy + the current attempt index +
//! a uniform random number in [0, 1), produce the next sleep
//! duration.  No I/O, no clock, no tokio.  Wrapped by the
//! retry loop in `core::retry_client`, which feeds it actual
//! random numbers and actual sleeps.
//!
//! Keeping this pure makes the schedule deterministically
//! testable: the integration test can pass fixed random values
//! and assert exact backoff bounds without relying on `tokio::time::pause`.

use std::time::Duration;

use crate::api::retry_config::GrpcRetryConfig;

/// Compute the backoff for the given attempt index (0-based:
/// attempt 0 is the first retry, i.e. the second call).
///
/// `random_unit` is a uniform value in `[0.0, 1.0)`.  Pass a
/// constant (e.g. 0.0 or 0.5) from tests to make the schedule
/// deterministic.
///
/// Uses the standard exponential schedule (`initial_backoff_ms`,
/// `max_backoff_ms`).  For the rate-limit track, use
/// [`rate_limit_backoff`] instead.
pub(crate) fn next_backoff(
    config:      &GrpcRetryConfig,
    attempt:     u32,
    random_unit: f64,
) -> Duration {
    debug_assert!((0.0..1.0).contains(&random_unit));
    exponential_jitter(
        config.initial_backoff_ms,
        config.max_backoff_ms,
        config.backoff_multiplier,
        config.jitter_factor,
        attempt,
        random_unit,
    )
}

/// Compute the backoff for a rate-limit retry.
///
/// When `retry_after_hint` is `Some`, that value is returned directly —
/// the transport extracted it from the upstream `Retry-After` header and
/// we should honour it exactly.  Otherwise, the computed exponential
/// schedule from the rate-limit config fields is used.
pub(crate) fn rate_limit_backoff(
    config:             &GrpcRetryConfig,
    attempt:            u32,
    retry_after_hint:   Option<Duration>,
    random_unit:        f64,
) -> Duration {
    if let Some(hint) = retry_after_hint {
        return hint;
    }
    debug_assert!((0.0..1.0).contains(&random_unit));
    exponential_jitter(
        config.rate_limit_initial_backoff_ms,
        config.rate_limit_max_backoff_ms,
        config.backoff_multiplier,
        config.jitter_factor,
        attempt,
        random_unit,
    )
}

fn exponential_jitter(
    initial_ms:   u64,
    max_ms:       u64,
    multiplier:   f64,
    jitter_factor: f64,
    attempt:      u32,
    random_unit:  f64,
) -> Duration {
    let base_ms   = (initial_ms as f64) * multiplier.powi(attempt as i32);
    let capped_ms = base_ms.min(max_ms as f64);

    // Symmetric jitter: jitter_factor=0.1 → multiplier in [0.9, 1.1).
    let jitter_mult = 1.0 - jitter_factor + (2.0 * jitter_factor * random_unit);
    let jittered_ms = capped_ms * jitter_mult;

    // Clamp post-jitter so jitter cannot push past the ceiling.
    let final_ms = jittered_ms.min(max_ms as f64).max(0.0);
    Duration::from_millis(final_ms.round() as u64)
}

/// Cheap PRNG for jitter — no `rand` crate dependency.
///
/// SplitMix64 (one round per call): well-distributed, fast,
/// deterministically seedable for tests.  We only need 53 bits
/// of randomness to fill an `f64` mantissa, so quality beyond
/// that doesn't matter.  Not for cryptographic use.
pub(crate) struct JitterRng {
    state: u64,
}

impl JitterRng {
    pub(crate) fn new(seed: u64) -> Self {
        // Avoid the all-zero state cycle.
        Self { state: seed.wrapping_add(0x9E3779B97F4A7C15) }
    }

    /// Seed from the current wall clock — used as a default
    /// when the caller doesn't pin a seed for testing.
    pub(crate) fn from_clock() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 ^ (d.as_secs() as u64).rotate_left(17))
            .unwrap_or(0);
        Self::new(nanos.wrapping_add(0xDEADBEEF))
    }

    /// Next uniform in `[0.0, 1.0)`.
    pub(crate) fn next_unit(&mut self) -> f64 {
        // SplitMix64 step.
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^= z >> 31;
        // Take the high 53 bits → uniform in [0, 1).
        ((z >> 11) as f64) * (1.0 / (1u64 << 53) as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> GrpcRetryConfig {
        GrpcRetryConfig::from_config(
            r#"
                max_attempts = 5
                initial_backoff_ms = 100
                backoff_multiplier = 2.0
                jitter_factor = 0.1
                max_backoff_ms = 5000
                rate_limit_max_attempts = 2
                rate_limit_initial_backoff_ms = 1000
                rate_limit_max_backoff_ms = 10000
            "#,
        )
        .unwrap()
    }

    fn cfg_no_jitter() -> GrpcRetryConfig {
        GrpcRetryConfig::from_config(
            r#"
                max_attempts = 5
                initial_backoff_ms = 100
                backoff_multiplier = 2.0
                jitter_factor = 0.0
                max_backoff_ms = 5000
                rate_limit_max_attempts = 2
                rate_limit_initial_backoff_ms = 1000
                rate_limit_max_backoff_ms = 10000
            "#,
        )
        .unwrap()
    }

    /// @covers: next_backoff — exponential growth with zero jitter.
    #[test]
    fn test_next_backoff_grows_exponentially_with_zero_jitter() {
        let c = cfg_no_jitter();
        assert_eq!(next_backoff(&c, 0, 0.0), Duration::from_millis(100));
        assert_eq!(next_backoff(&c, 1, 0.0), Duration::from_millis(200));
        assert_eq!(next_backoff(&c, 2, 0.0), Duration::from_millis(400));
        assert_eq!(next_backoff(&c, 3, 0.0), Duration::from_millis(800));
    }

    /// @covers: next_backoff — capped at max_backoff_ms.
    #[test]
    fn test_next_backoff_caps_at_max_backoff() {
        let c = cfg_no_jitter();
        let d = next_backoff(&c, 10, 0.0);
        assert_eq!(d, Duration::from_millis(5000));
    }

    /// @covers: next_backoff — jitter stays inside +/- jitter_factor band.
    #[test]
    fn test_next_backoff_jitter_stays_in_band() {
        let c = cfg();
        let lo = next_backoff(&c, 0, 0.0);
        let hi = next_backoff(&c, 0, 0.999);
        assert!(lo.as_millis() >= 90 && lo.as_millis() <= 100, "lo = {lo:?}");
        assert!(hi.as_millis() >= 100 && hi.as_millis() <= 110, "hi = {hi:?}");
    }

    /// @covers: next_backoff — jitter never exceeds the configured ceiling.
    #[test]
    fn test_next_backoff_jitter_cannot_overshoot_ceiling() {
        let c = cfg();
        let d = next_backoff(&c, 100, 0.999);
        assert!(d.as_millis() <= c.max_backoff_ms as u128, "{d:?} above ceiling");
    }

    /// @covers: rate_limit_backoff — uses Retry-After hint when present.
    #[test]
    fn test_rate_limit_backoff_uses_hint_when_present() {
        let c    = cfg_no_jitter();
        let hint = Some(Duration::from_secs(30));
        assert_eq!(rate_limit_backoff(&c, 0, hint, 0.0), Duration::from_secs(30));
    }

    /// @covers: rate_limit_backoff — computes from rate-limit fields when no hint.
    #[test]
    fn test_rate_limit_backoff_uses_rate_limit_track_without_hint() {
        let c = cfg_no_jitter();
        // attempt 0: 1000ms * 2.0^0 = 1000ms, no jitter
        let d = rate_limit_backoff(&c, 0, None, 0.0);
        assert_eq!(d, Duration::from_millis(1000));
    }

    /// @covers: rate_limit_backoff — capped at rate_limit_max_backoff_ms.
    #[test]
    fn test_rate_limit_backoff_caps_at_max() {
        let c = cfg_no_jitter();
        let d = rate_limit_backoff(&c, 20, None, 0.0);
        assert!(d.as_millis() <= c.rate_limit_max_backoff_ms as u128);
    }

    /// @covers: JitterRng — produces values in [0, 1).
    #[test]
    fn test_jitter_rng_produces_values_in_unit_interval() {
        let mut rng = JitterRng::new(42);
        for _ in 0..100 {
            let v = rng.next_unit();
            assert!((0.0..1.0).contains(&v), "{v} out of [0,1)");
        }
    }

    /// @covers: JitterRng — different seeds produce different sequences.
    #[test]
    fn test_jitter_rng_different_seeds_different_sequences() {
        let a = JitterRng::new(1).next_unit();
        let b = JitterRng::new(2).next_unit();
        assert_ne!(a, b);
    }

    /// @covers: JitterRng — same seed reproduces sequence.
    #[test]
    fn test_jitter_rng_same_seed_reproducible() {
        let mut x = JitterRng::new(7);
        let mut y = JitterRng::new(7);
        for _ in 0..16 {
            assert_eq!(x.next_unit(), y.next_unit());
        }
    }
}

#[cfg(test)]
mod dedicated_coverage {
    use super::JitterRng;

    /// @covers: new
    #[test]
    fn test_new_creates_rng_with_seed() {
        let mut rng = JitterRng::new(42);
        let v = rng.next_unit();
        assert!(v >= 0.0 && v < 1.0);
    }

    /// @covers: from_clock
    #[test]
    fn test_from_clock_creates_rng() {
        let mut rng = JitterRng::from_clock();
        let v = rng.next_unit();
        assert!(v >= 0.0 && v < 1.0);
    }

    /// @covers: next_unit
    #[test]
    fn test_next_unit_returns_value_in_range() {
        let mut rng = JitterRng::new(1);
        for _ in 0..10 {
            let v = rng.next_unit();
            assert!(v >= 0.0 && v < 1.0, "next_unit out of range: {v}");
        }
    }
}
