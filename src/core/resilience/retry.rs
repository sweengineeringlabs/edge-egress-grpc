//! Retry policy for transient gRPC errors.

use std::time::Duration;

use crate::api::port::{GrpcOutboundError};
use crate::api::value_object::GrpcStatusCode;

/// Policy controlling retry behaviour on transient gRPC errors.
///
/// Applied by [`super::resilient_client::ResilientGrpcClient`] before
/// recording a failure in the circuit breaker. Only errors returned by
/// [`RetryPolicy::is_retryable`] trigger a retry; permanent errors
/// (bad arguments, not found, internal client bugs) propagate immediately.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Total attempts including the first. `1` disables retry entirely.
    pub max_attempts: u32,
    /// Wait before the first retry.
    pub initial_backoff: Duration,
    /// Multiplier applied to the previous backoff on each successive attempt.
    pub backoff_multiplier: f64,
    /// Hard cap on the per-attempt backoff.
    pub max_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts:       3,
            initial_backoff:    Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_backoff:        Duration::from_millis(2_000),
        }
    }
}

impl RetryPolicy {
    /// `true` when `err` represents a transient condition worth retrying.
    ///
    /// Retryable:
    /// - `RESOURCE_EXHAUSTED` (8) — server capacity; may clear on retry
    /// - `UNAVAILABLE` (14) — server temporarily down or overloaded
    /// - Transport-level `Unavailable` — TCP unreachable, try again
    ///
    /// Not retryable (permanent or caller-driven):
    /// - `INTERNAL` (13) — ambiguous server error; not safe to retry blindly
    /// - `DEADLINE_EXCEEDED` (4) — budget already gone
    /// - `CANCELLED` — caller cancelled; respect the intent
    /// - All other status codes — bad request, auth, not found, etc.
    pub fn is_retryable(err: &GrpcOutboundError) -> bool {
        match err {
            GrpcOutboundError::Status(GrpcStatusCode::ResourceExhausted, _) => true,
            GrpcOutboundError::Status(GrpcStatusCode::Unavailable, _)       => true,
            GrpcOutboundError::Unavailable(_)                                => true,
            _ => false,
        }
    }

    /// Backoff duration before attempt number `retry_index` (0-based: 0 = first retry).
    ///
    /// Uses full jitter — a random value in `[0, min(base^n, max_backoff)]` —
    /// so concurrent callers spread their retries across the window rather than
    /// all hitting the downstream at exactly the same moment (thundering herd).
    pub fn backoff_for(&self, retry_index: u32) -> Duration {
        use rand::Rng;
        let ceiling_ms = (self.initial_backoff.as_millis() as f64
            * self.backoff_multiplier.powi(retry_index as i32))
            .min(self.max_backoff.as_millis() as f64) as u64;
        let jittered_ms = rand::thread_rng().gen_range(0..=ceiling_ms);
        Duration::from_millis(jittered_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: RetryPolicy::is_retryable — RESOURCE_EXHAUSTED is retryable.
    #[test]
    fn test_resource_exhausted_is_retryable() {
        let err = GrpcOutboundError::Status(GrpcStatusCode::ResourceExhausted, "oom".into());
        assert!(RetryPolicy::is_retryable(&err));
    }

    /// @covers: RetryPolicy::is_retryable — UNAVAILABLE (status) is retryable.
    #[test]
    fn test_unavailable_status_is_retryable() {
        let err = GrpcOutboundError::Status(GrpcStatusCode::Unavailable, "down".into());
        assert!(RetryPolicy::is_retryable(&err));
    }

    /// @covers: RetryPolicy::is_retryable — transport Unavailable is retryable.
    #[test]
    fn test_transport_unavailable_is_retryable() {
        assert!(RetryPolicy::is_retryable(&GrpcOutboundError::Unavailable("tcp".into())));
    }

    /// @covers: RetryPolicy::is_retryable — INTERNAL is not retryable.
    #[test]
    fn test_internal_status_is_not_retryable() {
        let err = GrpcOutboundError::Status(GrpcStatusCode::Internal, "bug".into());
        assert!(!RetryPolicy::is_retryable(&err));
    }

    /// @covers: RetryPolicy::is_retryable — Timeout is not retryable.
    #[test]
    fn test_timeout_is_not_retryable() {
        assert!(!RetryPolicy::is_retryable(&GrpcOutboundError::Timeout("late".into())));
    }

    /// @covers: RetryPolicy::is_retryable — Cancelled is not retryable.
    #[test]
    fn test_cancelled_is_not_retryable() {
        assert!(!RetryPolicy::is_retryable(&GrpcOutboundError::Cancelled("user".into())));
    }

    /// @covers: RetryPolicy::backoff_for — result is within [0, initial_backoff] on first retry.
    #[test]
    fn test_backoff_for_zero_is_within_initial_backoff_ceiling() {
        let p = RetryPolicy {
            initial_backoff:    Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_backoff:        Duration::from_secs(10),
            ..Default::default()
        };
        let b = p.backoff_for(0);
        assert!(b <= Duration::from_millis(100), "jittered backoff must not exceed ceiling");
    }

    /// @covers: RetryPolicy::backoff_for — ceiling grows with retry index.
    #[test]
    fn test_backoff_for_ceiling_grows_with_retry_index() {
        let p = RetryPolicy {
            initial_backoff:    Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_backoff:        Duration::from_secs(10),
            ..Default::default()
        };
        // Ceiling at index 1 = 200ms; result must be in [0, 200ms].
        let b = p.backoff_for(1);
        assert!(b <= Duration::from_millis(200));
    }

    /// @covers: RetryPolicy::backoff_for — result never exceeds max_backoff.
    #[test]
    fn test_backoff_for_never_exceeds_max_backoff() {
        let p = RetryPolicy {
            initial_backoff:    Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_backoff:        Duration::from_millis(150),
            ..Default::default()
        };
        for i in 0..20 {
            assert!(p.backoff_for(i) <= Duration::from_millis(150), "retry {i} exceeded max");
        }
    }
}
