//! Retry policy and RESOURCE_EXHAUSTED context discrimination.
//!
//! Mirrors the semantics of the standalone `swe-edge-egress-grpc-retry`
//! crate. The inline copy exists here because `swe-edge-egress-grpc`
//! cannot depend on that crate without creating a dependency cycle.

use std::time::Duration;

use crate::api::port::GrpcOutboundError;
use crate::api::value_object::GrpcStatusCode;

// ── RESOURCE_EXHAUSTED context ────────────────────────────────────────────────

/// Extract a `Retry-After` hint embedded in a gRPC error message by
/// the transport layer.
///
/// The transport embeds the HTTP `Retry-After` (or `x-ratelimit-reset-requests`)
/// header value as `[retry-after=Ns]` at the end of the `grpc-message` string
/// when it receives a `RESOURCE_EXHAUSTED` response.  This lets
/// [`RetryPolicy::decide`] honour the upstream reset window for rate-limit
/// errors without requiring a new error variant.
///
/// Returns `None` when no hint is present or it cannot be parsed.
pub fn parse_retry_after_hint(message: &str) -> Option<Duration> {
    let tag   = "[retry-after=";
    let start = message.find(tag)? + tag.len();
    let rest  = &message[start..];
    let end   = rest.find('s')?;
    let secs: u64 = rest[..end].parse().ok()?;
    Some(Duration::from_secs(secs))
}

/// Discriminates the cause of a `RESOURCE_EXHAUSTED` (gRPC 8) error.
///
/// | Context     | Cause                        | Correct response          |
/// |-------------|------------------------------|---------------------------|
/// | `Capacity`  | Server OOM / CPU saturation  | Retry standard track      |
/// | `RateLimit` | API rate-limit window full   | Retry rate-limit track    |
/// | `HardQuota` | Billing quota exhausted      | Do not retry              |
///
/// Classification inspects the `grpc-message` trailer for well-known
/// strings.  `Capacity` is the safe default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceExhaustedContext {
    /// Server capacity or OOM — may clear on retry after backoff.
    Capacity,
    /// API rate limit — the request window is full; retry after reset.
    RateLimit,
    /// Billing / quota hard cap — retry will not help.
    HardQuota,
}

/// Classify a `RESOURCE_EXHAUSTED` grpc-message into a context.
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
/// Callers interact through [`RetryPolicy::decide`], which returns a
/// [`RetryDecision`] combining the retry/no-retry choice with the
/// appropriate backoff for the specific error type.
///
/// Two backoff tracks:
/// - **Standard** — for `UNAVAILABLE` and `RESOURCE_EXHAUSTED(Capacity)`.
/// - **Rate-limit** — for `RESOURCE_EXHAUSTED(RateLimit)`, where the
///   reset window is typically seconds to minutes. Max attempts is also
///   separate. When the upstream embeds a `Retry-After` hint the transport
///   extracted from the response, that value overrides the computed backoff.
///
/// Both tracks use fractional jitter (`jitter_factor`) to prevent
/// thundering herd.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Total attempts including the first call. `1` disables retry entirely.
    pub max_attempts: u32,
    /// Initial backoff for capacity / unavailable errors.
    pub initial_backoff: Duration,
    /// Exponential growth factor applied per retry index.
    pub backoff_multiplier: f64,
    /// Jitter as a fraction of the computed backoff (0.0 = none, 0.1 = ±10%).
    pub jitter_factor: f64,
    /// Hard cap on the per-attempt backoff for capacity / unavailable errors.
    pub max_backoff: Duration,
    /// Max attempts specifically for rate-limit `RESOURCE_EXHAUSTED`.
    pub rate_limit_max_attempts: u32,
    /// Initial backoff for rate-limit errors (typically longer than capacity).
    pub rate_limit_initial_backoff: Duration,
    /// Hard cap on rate-limit backoff.
    pub rate_limit_max_backoff: Duration,
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
    /// - `RESOURCE_EXHAUSTED(RateLimit)` — rate-limit track
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
                            // Honour Retry-After header embedded by the transport;
                            // fall back to computed backoff when absent.
                            let backoff = parse_retry_after_hint(msg).unwrap_or_else(|| {
                                self.jittered(
                                    self.rate_limit_initial_backoff,
                                    self.rate_limit_max_backoff,
                                    retry_index,
                                )
                            });
                            RetryDecision::Retry(backoff)
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
            | GrpcOutboundError::Unavailable(_)
            | GrpcOutboundError::ConnectionFailed(_) => {
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

    /// Fractional-jitter backoff in `[base*(1-f), min(base*(1+f), max)]`.
    ///
    /// `jitter_factor = 0.1` → ±10% around the computed exponential base,
    /// capped at `max`. Same algorithm as `swe-edge-egress-grpc-retry`.
    fn jittered(&self, initial: Duration, max: Duration, retry_index: u32) -> Duration {
        let base_ms = (initial.as_millis() as f64
            * self.backoff_multiplier.powi(retry_index as i32))
            .min(max.as_millis() as f64);
        let random_unit = self.next_random();
        let jitter_mult = 1.0 - self.jitter_factor + (2.0 * self.jitter_factor * random_unit);
        let jittered = (base_ms * jitter_mult).min(max.as_millis() as f64).max(0.0);
        Duration::from_millis(jittered.round() as u64)
    }

    fn next_random(&self) -> f64 {
        use rand::Rng;
        rand::thread_rng().gen_range(0.0_f64..1.0)
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
        assert_eq!(classify_resource_exhausted("quota exceeded"),    ResourceExhaustedContext::HardQuota);
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

    fn policy() -> RetryPolicy {
        RetryPolicy {
            max_attempts:               3,
            initial_backoff:            Duration::ZERO,
            backoff_multiplier:         1.0,
            jitter_factor:              0.0,
            max_backoff:                Duration::ZERO,
            rate_limit_max_attempts:    2,
            rate_limit_initial_backoff: Duration::ZERO,
            rate_limit_max_backoff:     Duration::ZERO,
        }
    }

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

    /// @covers: decide — ConnectionFailed → Retry (transport-level transient).
    #[test]
    fn test_decide_connection_failed_retries() {
        assert!(matches!(
            policy().decide(&GrpcOutboundError::ConnectionFailed("rst".into()), 0),
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

    /// @covers: parse_retry_after_hint — extracts seconds from embedded tag.
    #[test]
    fn test_parse_retry_after_hint_extracts_seconds() {
        assert_eq!(
            parse_retry_after_hint("rate limit exceeded [retry-after=30s]"),
            Some(Duration::from_secs(30))
        );
    }

    /// @covers: parse_retry_after_hint — returns None when tag absent.
    #[test]
    fn test_parse_retry_after_hint_returns_none_when_absent() {
        assert_eq!(parse_retry_after_hint("rate limit exceeded"), None);
        assert_eq!(parse_retry_after_hint(""), None);
    }

    /// @covers: decide — RateLimit with Retry-After hint uses the hint duration.
    #[test]
    fn test_decide_rate_limit_uses_retry_after_hint_when_present() {
        let p   = policy();
        let msg = "rate limit exceeded [retry-after=30s]";
        let d   = p.decide(&resource_exhausted(msg), 0);
        match d {
            RetryDecision::Retry(backoff) => {
                assert_eq!(backoff, Duration::from_secs(30),
                    "must use Retry-After value exactly, not computed backoff");
            }
            RetryDecision::DoNotRetry => panic!("expected Retry"),
        }
    }
}
