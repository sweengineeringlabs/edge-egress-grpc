//! Resilience configuration — retry + circuit breaker as a first-class
//! channel option.
//!
//! Uses millisecond integer fields for retry and seconds for the breaker
//! cool-down so the struct maps directly to TOML without serde helpers:
//!
//! ```toml
//! [channel.resilience]
//! max_attempts                  = 3
//! initial_backoff_ms            = 100
//! backoff_multiplier            = 2.0
//! jitter_factor                 = 0.1
//! max_backoff_ms                = 2000
//! rate_limit_max_attempts       = 2
//! rate_limit_initial_backoff_ms = 1000
//! rate_limit_max_backoff_ms     = 10000
//! failure_threshold             = 5
//! cool_down_seconds             = 10
//! half_open_probe_count         = 1
//! ```

use serde::{Deserialize, Serialize};

/// Resilience policy for a single outbound gRPC channel.
///
/// Consumed by [`crate::saf::create_transport_from_config`] to compose
/// a [`swe_edge_egress_grpc_retry::GrpcRetryClient`] and
/// [`swe_edge_egress_grpc_breaker::GrpcBreakerClient`] around the
/// bare [`crate::TonicGrpcClient`] when present on a
/// [`super::GrpcChannelConfig`].
///
/// ## Retry tracks
///
/// Two independent tracks because rate-limit and capacity errors recover
/// on different time scales:
///
/// - **Standard** — for `UNAVAILABLE`, `ConnectionFailed`, and
///   `RESOURCE_EXHAUSTED(Capacity)`: fast exponential backoff.
/// - **Rate-limit** — for `RESOURCE_EXHAUSTED(RateLimit)`: slower,
///   separate attempt ceiling. When the upstream response carries a
///   `Retry-After` header the transport has embedded as
///   `[retry-after=Ns]`, that value overrides the computed backoff.
/// - **HardQuota** (`RESOURCE_EXHAUSTED` with billing / quota message):
///   never retried regardless of `max_attempts`.
///
/// ## Circuit breaker
///
/// Opens after `failure_threshold` consecutive post-retry transport
/// failures (`UNAVAILABLE`, `INTERNAL`, `ConnectionFailed`).  Stays
/// open for `cool_down_seconds`, then allows `half_open_probe_count`
/// consecutive probe successes to close it.
///
/// Note: `RESOURCE_EXHAUSTED` is **not** a breaker failure; it is
/// handled entirely by the retry layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceConfig {
    // ── Standard retry ────────────────────────────────────────────────────────
    /// Total attempts including the first call. `1` disables retry.
    pub max_attempts: u32,
    /// Wait before the first standard retry (ms).
    pub initial_backoff_ms: u64,
    /// Exponential growth factor per retry index.
    pub backoff_multiplier: f64,
    /// Jitter as a fraction of the computed backoff (0.0 = none, 0.1 = ±10%).
    pub jitter_factor: f64,
    /// Hard cap on standard retry backoff (ms).
    pub max_backoff_ms: u64,

    // ── Rate-limit retry track ────────────────────────────────────────────────
    /// Max attempts on the rate-limit track (usually lower than `max_attempts`).
    pub rate_limit_max_attempts: u32,
    /// Wait before the first rate-limit retry (ms). Overridden by
    /// `[retry-after=Ns]` hint when present.
    pub rate_limit_initial_backoff_ms: u64,
    /// Hard cap on rate-limit backoff (ms).
    pub rate_limit_max_backoff_ms: u64,

    // ── Circuit breaker ───────────────────────────────────────────────────────
    /// Consecutive post-retry transport failures before opening the circuit.
    pub failure_threshold: u32,
    /// Seconds the circuit stays open before probing.
    pub cool_down_seconds: u64,
    /// Consecutive probe successes required in HalfOpen to close.
    pub half_open_probe_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ResilienceConfig {
        ResilienceConfig {
            max_attempts:                  3,
            initial_backoff_ms:            100,
            backoff_multiplier:            2.0,
            jitter_factor:                 0.1,
            max_backoff_ms:                2_000,
            rate_limit_max_attempts:       2,
            rate_limit_initial_backoff_ms: 1_000,
            rate_limit_max_backoff_ms:     10_000,
            failure_threshold:             5,
            cool_down_seconds:             10,
            half_open_probe_count:         1,
        }
    }

    #[test]
    fn test_round_trips_through_toml() {
        let original = sample();
        let encoded  = toml::to_string(&original).expect("serialize");
        let restored: ResilienceConfig = toml::from_str(&encoded).expect("deserialize");
        assert_eq!(restored.max_attempts,       original.max_attempts);
        assert_eq!(restored.failure_threshold,  original.failure_threshold);
        assert_eq!(restored.cool_down_seconds,  original.cool_down_seconds);
        assert_eq!(restored.half_open_probe_count, original.half_open_probe_count);
        assert_eq!(restored.rate_limit_max_attempts, original.rate_limit_max_attempts);
    }

    #[test]
    fn test_all_fields_survive_round_trip() {
        let s = sample();
        let t = toml::to_string(&s).unwrap();
        let r: ResilienceConfig = toml::from_str(&t).unwrap();
        assert_eq!(r.initial_backoff_ms,            s.initial_backoff_ms);
        assert_eq!(r.backoff_multiplier,             s.backoff_multiplier);
        assert_eq!(r.jitter_factor,                  s.jitter_factor);
        assert_eq!(r.max_backoff_ms,                 s.max_backoff_ms);
        assert_eq!(r.rate_limit_initial_backoff_ms,  s.rate_limit_initial_backoff_ms);
        assert_eq!(r.rate_limit_max_backoff_ms,      s.rate_limit_max_backoff_ms);
    }
}
