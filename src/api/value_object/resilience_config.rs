//! Resilience configuration — retry + circuit breaker as a first-class
//! channel option.
//!
//! Uses millisecond integer fields rather than `Duration` so the struct
//! maps directly to TOML without serde helper attributes:
//!
//! ```toml
//! [channel.resilience]
//! max_attempts            = 3
//! initial_backoff_ms      = 100
//! backoff_multiplier      = 2.0
//! max_backoff_ms          = 2000
//! rate_limit_max_attempts = 2
//! rate_limit_initial_backoff_ms = 1000
//! rate_limit_max_backoff_ms     = 10000
//! failure_threshold       = 5
//! open_duration_ms        = 10000
//! ```

use serde::{Deserialize, Serialize};

/// Resilience policy for a single outbound gRPC channel.
///
/// Consumed by [`crate::saf::create_transport_from_config`] to wrap the
/// bare [`crate::TonicGrpcClient`] with [`crate::ResilientGrpcClient`]
/// when present on a [`super::GrpcChannelConfig`].
///
/// ## Retry tracks
///
/// Two independent tracks because rate-limit and capacity errors have
/// different recovery times:
///
/// - **Standard** — for `UNAVAILABLE` and `RESOURCE_EXHAUSTED(Capacity)`:
///   fast exponential backoff (`initial_backoff_ms`, `max_backoff_ms`).
/// - **Rate-limit** — for `RESOURCE_EXHAUSTED(RateLimit)`: slower,
///   separate attempt ceiling (`rate_limit_*` fields). When the upstream
///   response carries a `Retry-After` header, that value overrides the
///   computed backoff entirely.
///
/// ## Circuit breaker
///
/// Opens after `failure_threshold` consecutive post-retry failures,
/// stays open for `open_duration_ms`, then allows one probe.
/// Set `failure_threshold = 0` to disable the circuit breaker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceConfig {
    // ── Standard retry (capacity / unavailable) ───────────────────────────
    /// Total attempts including the first call. `1` disables retry.
    pub max_attempts: u32,
    /// Wait before the first standard retry (ms).
    pub initial_backoff_ms: u64,
    /// Exponential growth factor per retry index.
    pub backoff_multiplier: f64,
    /// Hard cap on standard retry backoff (ms).
    pub max_backoff_ms: u64,

    // ── Rate-limit retry track ────────────────────────────────────────────
    /// Max attempts on the rate-limit track. Often lower than `max_attempts`.
    pub rate_limit_max_attempts: u32,
    /// Wait before the first rate-limit retry (ms). Overridden by
    /// `Retry-After` header when present.
    pub rate_limit_initial_backoff_ms: u64,
    /// Hard cap on rate-limit backoff (ms).
    pub rate_limit_max_backoff_ms: u64,

    // ── Circuit breaker ───────────────────────────────────────────────────
    /// Consecutive post-retry failures before opening the circuit.
    /// `0` disables the circuit breaker.
    pub failure_threshold: u32,
    /// Duration the circuit stays open before probing (ms).
    pub open_duration_ms: u64,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            max_attempts:                  3,
            initial_backoff_ms:            100,
            backoff_multiplier:            2.0,
            max_backoff_ms:                2_000,
            rate_limit_max_attempts:       2,
            rate_limit_initial_backoff_ms: 1_000,
            rate_limit_max_backoff_ms:     10_000,
            failure_threshold:             5,
            open_duration_ms:              10_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: ResilienceConfig::default — sensible production defaults.
    #[test]
    fn test_default_values_are_production_safe() {
        let c = ResilienceConfig::default();
        assert_eq!(c.max_attempts, 3);
        assert_eq!(c.initial_backoff_ms, 100);
        assert_eq!(c.failure_threshold, 5);
        assert_eq!(c.open_duration_ms, 10_000);
        assert!(c.rate_limit_max_attempts < c.max_attempts,
            "rate-limit attempts should be <= standard attempts");
    }

    /// @covers: ResilienceConfig — round-trips through serde (using toml).
    #[test]
    fn test_round_trips_through_toml() {
        let original = ResilienceConfig::default();
        let encoded = toml::to_string(&original).expect("serialize");
        let restored: ResilienceConfig = toml::from_str(&encoded).expect("deserialize");
        assert_eq!(restored.max_attempts, original.max_attempts);
        assert_eq!(restored.failure_threshold, original.failure_threshold);
    }
}
