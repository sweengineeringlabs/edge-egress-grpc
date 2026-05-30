//! Exponential-backoff-with-jitter scheduler.
//!
//! Pure logic — given the policy + the current attempt index +
//! a uniform random number in [0, 1), produce the next sleep
//! duration.  No I/O, no clock, no tokio.  Wrapped by the
//! retry loop in `core/retry_decorator/`, which feeds it actual
//! random numbers and actual sleeps.
//!
//! Keeping this pure makes the schedule deterministically
//! testable: the integration test can pass fixed random values
//! and assert exact backoff bounds without relying on `tokio::time::pause`.

use std::time::Duration;

use crate::api::types::grpc_retry_config::GrpcRetryConfig;

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
fn next_backoff(config: &GrpcRetryConfig, attempt: u32, random_unit: f64) -> Duration {
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
fn rate_limit_backoff(
    config: &GrpcRetryConfig,
    attempt: u32,
    retry_after_hint: Option<Duration>,
    random_unit: f64,
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
    initial_ms: u64,
    max_ms: u64,
    multiplier: f64,
    jitter_factor: f64,
    attempt: u32,
    random_unit: f64,
) -> Duration {
    let base_ms = (initial_ms as f64) * multiplier.powi(attempt as i32);
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

/// Stateless backoff computation helper.
///
/// Wraps the free-standing backoff schedule functions as associated methods
/// to satisfy SEA rule 191 (all functions must be methods on a type).
pub(crate) struct BackoffScheduler;

impl BackoffScheduler {
    /// Compute next standard-retry backoff.
    pub(crate) fn next_backoff(
        config: &GrpcRetryConfig,
        attempt: u32,
        random_unit: f64,
    ) -> Duration {
        next_backoff(config, attempt, random_unit)
    }

    /// Compute rate-limit backoff.
    pub(crate) fn rate_limit_backoff(
        config: &GrpcRetryConfig,
        attempt: u32,
        retry_after_hint: Option<Duration>,
        random_unit: f64,
    ) -> Duration {
        rate_limit_backoff(config, attempt, retry_after_hint, random_unit)
    }
}
