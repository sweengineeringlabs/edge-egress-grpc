//! Retry policy and RESOURCE_EXHAUSTED context discrimination.

use std::time::Duration;

use crate::api::port::GrpcOutboundError;
use crate::api::value_object::GrpcStatusCode;

// ── RESOURCE_EXHAUSTED context ────────────────────────────────────────────────

/// Discriminates the cause of a `RESOURCE_EXHAUSTED` (gRPC 8) error.
///
/// The same status code covers three distinct situations that require
/// different retry strategies:
///
/// | Context | Cause | Correct response |
/// |---------|-------|-----------------|
/// | `Capacity` | Server OOM / CPU saturation | Retry with standard backoff; let the pool recover |
/// | `RateLimit` | API rate limit window exceeded | Retry with longer backoff; respect the reset window |
/// | `HardQuota` | Billing quota exhausted | Do not retry; surface to operator |
///
/// Classification inspects the `grpc-message` trailer for well-known
/// strings — no caller annotation required, no proto changes needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceExhaustedContext {
    /// Server capacity or OOM — may clear on retry after backoff.
    Capacity,
    /// API rate limit — the request window is full; retry after it resets.
    RateLimit,
    /// Billing / quota hard cap — retry will not help; escalate to operator.
    HardQuota,
}

/// Classify a `RESOURCE_EXHAUSTED` gRPC message into a [`ResourceExhaustedContext`].
///
/// Matched strings are intentionally broad and case-insensitive to handle
/// different upstream vendors (Anthropic, Google, OpenAI) without per-vendor
/// branches.  `Capacity` is the safe default — it triggers a retry, which is
/// always better than silently dropping the request if we classified wrong.
pub fn classify_resource_exhausted(message: &str) -> ResourceExhaustedContext {
    let msg = message.to_ascii_lowercase();
    if msg.contains("quota") || msg.contains("billing") || msg.contains("plan limit") {
        ResourceExhaustedContext::HardQuota
    } else if msg.contains("rate") || msg.contains("too many requests") || msg.contains("throttl") {
        ResourceExhaustedContext::RateLimit
    } else {
        ResourceExhaustedContext::Capacity
    }
}

// ── RetryDecision ─────────────────────────────────────────────────────────────

/// What the retry policy decided for one error on one attempt.
#[derive(Debug)]
pub enum RetryDecision {
    /// Retry after sleeping for the given duration.
    Retry(Duration),
    /// Propagate the error immediately — do not retry.
    DoNotRetry,
}

// ── RetryPolicy ───────────────────────────────────────────────────────────────

/// Policy controlling retry behaviour on transient gRPC errors.
///
/// Callers interact with the policy through a single entry point:
/// [`RetryPolicy::decide`], which returns a [`RetryDecision`] combining
/// the retry/no-retry choice with the appropriate backoff for the specific
/// error type. This replaces the earlier split between `is_retryable` and
/// `backoff_for`, making it impossible to apply the wrong backoff for a
/// given context.
///
/// Two backoff tracks:
/// - **Standard** (`initial_backoff` / `backoff_multiplier` / `max_backoff`) —
///   for `UNAVAILABLE` and `RESOURCE_EXHAUSTED(Capacity)`.
/// - **Rate-limit** (`rate_limit_initial_backoff` / `rate_limit_max_backoff`) —
///   for `RESOURCE_EXHAUSTED(RateLimit)`, where the reset window is typically
///   seconds to minutes. Max attempts is also separate so the rate-limit path
///   can exhaust fewer overall retries.
///
/// Both tracks use full jitter to prevent thundering herd.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Total attempts including the first call. `1` disables retry entirely.
    pub max_attempts: u32,
    /// Initial backoff for capacity / unavailable errors.
    pub initial_backoff: Duration,
    /// Exponential growth factor applied per retry index.
    pub backoff_multiplier: f64,
    /// Hard cap on the per-attempt backoff for capacity / unavailable errors.
    pub max_backoff: Duration,
    /// Max attempts specifically for rate-limit `RESOURCE_EXHAUSTED`.
    /// Often lower than `max_attempts` since rate-limit retries are expensive.
    pub rate_limit_max_attempts: u32,
    /// Initial backoff for rate-limit errors (typically longer than capacity).
    pub rate_limit_initial_backoff: Duration,
    /// Hard cap on rate-limit backoff.
    pub rate_limit_max_backoff: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts:            3,
            initial_backoff:         Duration::from_millis(100),
            backoff_multiplier:      2.0,
            max_backoff:             Duration::from_millis(2_000),
            rate_limit_max_attempts: 2,
            rate_limit_initial_backoff: Duration::from_secs(1),
            rate_limit_max_backoff:  Duration::from_secs(10),
        }
    }
}

impl RetryPolicy {
    /// Decide whether to retry and, if so, how long to wait.
    ///
    /// Returns [`RetryDecision::DoNotRetry`] for:
    /// - Permanent errors (`INTERNAL`, `INVALID_ARGUMENT`, `NOT_FOUND`, …)
    /// - `RESOURCE_EXHAUSTED(HardQuota)` — retry would not help
    /// - `DEADLINE_EXCEEDED` / `CANCELLED` — budget gone or caller aborted
    /// - Attempts at or beyond the relevant `max_attempts` ceiling
    ///
    /// Returns [`RetryDecision::Retry(backoff)`] for:
    /// - `RESOURCE_EXHAUSTED(Capacity)` — standard track
    /// - `RESOURCE_EXHAUSTED(RateLimit)` — rate-limit track (longer backoff)
    /// - `UNAVAILABLE` (status or transport) — standard track
    ///
    /// `retry_index` is 0-based (0 = deciding before the first retry).
    pub fn decide(&self, err: &GrpcOutboundError, retry_index: u32) -> RetryDecision {
        match err {
            GrpcOutboundError::Status(GrpcStatusCode::ResourceExhausted, msg) => {
                match classify_resource_exhausted(msg) {
                    ResourceExhaustedContext::HardQuota => RetryDecision::DoNotRetry,
                    ResourceExhaustedContext::RateLimit => {
                        if retry_index >= self.rate_limit_max_attempts {
                            RetryDecision::DoNotRetry
                        } else {
                            RetryDecision::Retry(self.jittered(
                                self.rate_limit_initial_backoff,
                                self.rate_limit_max_backoff,
                                retry_index,
                            ))
                        }
                    }
                    ResourceExhaustedContext::Capacity => {
                        if retry_index >= self.max_attempts {
                            RetryDecision::DoNotRetry
                        } else {
                            RetryDecision::Retry(self.jittered(
                                self.initial_backoff,
                                self.max_backoff,
                                retry_index,
                            ))
                        }
                    }
                }
            }
            GrpcOutboundError::Status(GrpcStatusCode::Unavailable, _)
            | GrpcOutboundError::Unavailable(_) => {
                if retry_index >= self.max_attempts {
                    RetryDecision::DoNotRetry
                } else {
                    RetryDecision::Retry(self.jittered(
                        self.initial_backoff,
                        self.max_backoff,
                        retry_index,
                    ))
                }
            }
            _ => RetryDecision::DoNotRetry,
        }
    }

    /// Full-jitter backoff in `[0, min(initial * multiplier^index, max)]`.
    fn jittered(&self, initial: Duration, max: Duration, retry_index: u32) -> Duration {
        use rand::Rng;
        let ceiling_ms = (initial.as_millis() as f64
            * self.backoff_multiplier.powi(retry_index as i32))
            .min(max.as_millis() as f64) as u64;
        Duration::from_millis(rand::thread_rng().gen_range(0..=ceiling_ms))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── classify_resource_exhausted ───────────────────────────────────────────

    /// @covers: classify_resource_exhausted — rate-limit keywords → RateLimit.
    #[test]
    fn test_classify_rate_limit_by_keyword() {
        assert_eq!(classify_resource_exhausted("rate limit exceeded"), ResourceExhaustedContext::RateLimit);
        assert_eq!(classify_resource_exhausted("too many requests"),   ResourceExhaustedContext::RateLimit);
        assert_eq!(classify_resource_exhausted("throttled"),           ResourceExhaustedContext::RateLimit);
        assert_eq!(classify_resource_exhausted("RATE_LIMIT_ERROR"),    ResourceExhaustedContext::RateLimit);
    }

    /// @covers: classify_resource_exhausted — quota keywords → HardQuota.
    #[test]
    fn test_classify_hard_quota_by_keyword() {
        assert_eq!(classify_resource_exhausted("quota exceeded"),   ResourceExhaustedContext::HardQuota);
        assert_eq!(classify_resource_exhausted("billing limit hit"), ResourceExhaustedContext::HardQuota);
        assert_eq!(classify_resource_exhausted("plan limit reached"), ResourceExhaustedContext::HardQuota);
    }

    /// @covers: classify_resource_exhausted — unknown message defaults to Capacity.
    #[test]
    fn test_classify_unknown_defaults_to_capacity() {
        assert_eq!(classify_resource_exhausted("out of memory"),   ResourceExhaustedContext::Capacity);
        assert_eq!(classify_resource_exhausted("server overloaded"), ResourceExhaustedContext::Capacity);
        assert_eq!(classify_resource_exhausted(""),                ResourceExhaustedContext::Capacity);
    }

    // ── RetryPolicy::decide ───────────────────────────────────────────────────

    fn policy() -> RetryPolicy { RetryPolicy::default() }

    fn resource_exhausted(msg: &str) -> GrpcOutboundError {
        GrpcOutboundError::Status(GrpcStatusCode::ResourceExhausted, msg.into())
    }

    /// @covers: decide — RESOURCE_EXHAUSTED(Capacity) → Retry on first attempt.
    #[test]
    fn test_decide_capacity_exhausted_retries() {
        assert!(matches!(policy().decide(&resource_exhausted("oom"), 0), RetryDecision::Retry(_)));
    }

    /// @covers: decide — RESOURCE_EXHAUSTED(RateLimit) → Retry with rate-limit track.
    #[test]
    fn test_decide_rate_limit_retries_on_rate_limit_track() {
        let d = policy().decide(&resource_exhausted("rate limit exceeded"), 0);
        assert!(matches!(d, RetryDecision::Retry(_)), "rate-limit should retry");
    }

    /// @covers: decide — RESOURCE_EXHAUSTED(HardQuota) → DoNotRetry immediately.
    #[test]
    fn test_decide_hard_quota_does_not_retry() {
        assert!(matches!(
            policy().decide(&resource_exhausted("quota exceeded"), 0),
            RetryDecision::DoNotRetry
        ));
    }

    /// @covers: decide — RESOURCE_EXHAUSTED(RateLimit) stops after rate_limit_max_attempts.
    #[test]
    fn test_decide_rate_limit_stops_at_rate_limit_max_attempts() {
        let p = policy();
        // At index = rate_limit_max_attempts the policy should stop.
        assert!(matches!(
            p.decide(&resource_exhausted("rate limit"), p.rate_limit_max_attempts),
            RetryDecision::DoNotRetry
        ));
    }

    /// @covers: decide — UNAVAILABLE → Retry on standard track.
    #[test]
    fn test_decide_unavailable_retries() {
        let err = GrpcOutboundError::Status(GrpcStatusCode::Unavailable, "down".into());
        assert!(matches!(policy().decide(&err, 0), RetryDecision::Retry(_)));
    }

    /// @covers: decide — transport Unavailable → Retry.
    #[test]
    fn test_decide_transport_unavailable_retries() {
        assert!(matches!(
            policy().decide(&GrpcOutboundError::Unavailable("tcp".into()), 0),
            RetryDecision::Retry(_)
        ));
    }

    /// @covers: decide — INTERNAL → DoNotRetry.
    #[test]
    fn test_decide_internal_does_not_retry() {
        let err = GrpcOutboundError::Status(GrpcStatusCode::Internal, "bug".into());
        assert!(matches!(policy().decide(&err, 0), RetryDecision::DoNotRetry));
    }

    /// @covers: decide — Timeout → DoNotRetry.
    #[test]
    fn test_decide_timeout_does_not_retry() {
        assert!(matches!(
            policy().decide(&GrpcOutboundError::Timeout("late".into()), 0),
            RetryDecision::DoNotRetry
        ));
    }

    /// @covers: decide — Cancelled → DoNotRetry.
    #[test]
    fn test_decide_cancelled_does_not_retry() {
        assert!(matches!(
            policy().decide(&GrpcOutboundError::Cancelled("user".into()), 0),
            RetryDecision::DoNotRetry
        ));
    }

    /// @covers: decide — capacity retry stops at max_attempts.
    #[test]
    fn test_decide_capacity_stops_at_max_attempts() {
        let p = policy();
        assert!(matches!(
            p.decide(&resource_exhausted("oom"), p.max_attempts),
            RetryDecision::DoNotRetry
        ));
    }

    /// @covers: RetryPolicy::jittered — result never exceeds max.
    #[test]
    fn test_jittered_never_exceeds_max() {
        let p = policy();
        for i in 0..20 {
            let b = p.jittered(Duration::from_millis(100), Duration::from_millis(150), i);
            assert!(b <= Duration::from_millis(150), "retry {i} exceeded max");
        }
    }
}
