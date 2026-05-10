//! Per-status retry decision.  Single source of truth for which
//! gRPC status codes the decorator will retry.
//!
//! The policy is hand-written, not config-driven, because the
//! retryable set is a property of gRPC's semantic contract — not
//! a tunable knob.  Specifically:
//!
//! - `Unavailable` and `ResourceExhausted` are retryable per the
//!   gRPC retry whitepaper (the latter with a longer backoff,
//!   surfaced via [`RetryDecision::is_rate_limit`]).
//! - `ResourceExhausted(HardQuota)` is NOT retried — a billing
//!   cap won't clear on retry.
//! - `Unauthenticated` and `PermissionDenied` MUST NOT be
//!   retried — a bad token won't become good by trying again,
//!   and silent retries hide auth failures from the caller.
//! - `DeadlineExceeded` must not be retried — the caller's
//!   deadline already counts the retry budget; re-issuing
//!   guarantees a second deadline trip.
//! - `Internal` is not retried — server bug, retrying just
//!   amplifies the bug and burns the deadline.

use std::time::Duration;

use swe_edge_egress_grpc::{GrpcOutboundError, GrpcStatusCode};

// ── ResourceExhaustedContext ─────────────────────────────────────────────────

/// Discriminates the cause of a `RESOURCE_EXHAUSTED` (gRPC 8) error.
///
/// The same status code covers three situations that require different
/// retry strategies:
///
/// | Context     | Cause                        | Correct response          |
/// |-------------|------------------------------|---------------------------|
/// | `Capacity`  | Server OOM / CPU saturation  | Retry standard track      |
/// | `RateLimit` | API rate-limit window full   | Retry rate-limit track    |
/// | `HardQuota` | Billing quota exhausted      | Do not retry; escalate    |
///
/// Classification inspects the `grpc-message` string for well-known
/// keywords. `Capacity` is the safe default — it triggers a retry,
/// which is always better than silently dropping the request.
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

/// Extract a `Retry-After` hint embedded in a gRPC error message.
///
/// The transport embeds the HTTP `Retry-After` (or
/// `x-ratelimit-reset-requests`) header value as `[retry-after=Ns]`
/// at the end of the `grpc-message` when it receives a
/// `RESOURCE_EXHAUSTED` response.  This lets the retry loop honour
/// the upstream reset window for rate-limit errors without requiring
/// a new error variant or extra fields.
///
/// Returns `None` when no hint is present or when the value cannot
/// be parsed as a whole number of seconds.
pub fn parse_retry_after_hint(message: &str) -> Option<Duration> {
    let tag = "[retry-after=";
    let start = message.find(tag)? + tag.len();
    let rest = &message[start..];
    let end = rest.find('s')?;
    let secs: u64 = rest[..end].parse().ok()?;
    Some(Duration::from_secs(secs))
}

// ── RetryDecision ─────────────────────────────────────────────────────────────

/// Decision returned by [`classify`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryDecision {
    /// Treat as success — return to caller, no retry.
    Success,
    /// Retry-eligible failure — use the standard backoff schedule.
    Retry,
    /// Retry-eligible `ResourceExhausted(RateLimit)` — use the
    /// rate-limit backoff track (slower, respects Retry-After hints).
    RetryRateLimit,
    /// Terminal failure — surface to caller without retrying.
    Terminal,
}

impl RetryDecision {
    /// True if the decision indicates the call should be re-issued.
    pub fn should_retry(self) -> bool {
        matches!(self, RetryDecision::Retry | RetryDecision::RetryRateLimit)
    }

    /// True if the decision is the rate-limit variant.
    pub fn is_rate_limit(self) -> bool {
        matches!(self, RetryDecision::RetryRateLimit)
    }
}

/// Classify an outbound result into a retry decision.
///
/// Mapping table (for non-`Ok` outcomes):
///
/// | Variant                                            | Decision          |
/// |----------------------------------------------------|-------------------|
/// | `Status(Unavailable, _)` / `Unavailable(_)`        | `Retry`           |
/// | `Status(ResourceExhausted, _)` — RateLimit msg     | `RetryRateLimit`  |
/// | `Status(ResourceExhausted, _)` — Capacity msg      | `Retry`           |
/// | `Status(ResourceExhausted, _)` — HardQuota msg     | `Terminal`        |
/// | `Status(Unauthenticated, _)`                       | `Terminal`        |
/// | `Status(PermissionDenied, _)`                      | `Terminal`        |
/// | `Status(DeadlineExceeded, _)` / `Timeout(_)`       | `Terminal`        |
/// | `Status(Internal, _)` / `Internal(_)`              | `Terminal`        |
/// | `ConnectionFailed(_)`                              | `Retry`           |
/// | `Cancelled(_)` / `Status(Cancelled, _)`            | `Terminal`        |
/// | other `Status(_, _)`                               | `Terminal`        |
///
/// `ConnectionFailed` is treated as `Retry` because it's a
/// transport-level transient (DNS hiccup, TCP RST during a rolling
/// deploy) matching canonical `Unavailable` gRPC semantics.
pub fn classify<T>(result: &Result<T, GrpcOutboundError>) -> RetryDecision {
    let err = match result {
        Ok(_) => return RetryDecision::Success,
        Err(e) => e,
    };
    match err {
        GrpcOutboundError::Status(code, msg) => match code {
            GrpcStatusCode::Unavailable => RetryDecision::Retry,
            GrpcStatusCode::ResourceExhausted => match classify_resource_exhausted(msg) {
                ResourceExhaustedContext::HardQuota => RetryDecision::Terminal,
                ResourceExhaustedContext::RateLimit => RetryDecision::RetryRateLimit,
                ResourceExhaustedContext::Capacity => RetryDecision::Retry,
            },
            // Explicit non-retryable variants — listed here so
            // adding a new variant on `GrpcStatusCode` surfaces
            // as a missing arm, not a silent default.
            GrpcStatusCode::Unauthenticated
            | GrpcStatusCode::PermissionDenied
            | GrpcStatusCode::DeadlineExceeded
            | GrpcStatusCode::Internal
            | GrpcStatusCode::Cancelled
            | GrpcStatusCode::Ok
            | GrpcStatusCode::Unknown
            | GrpcStatusCode::InvalidArgument
            | GrpcStatusCode::NotFound
            | GrpcStatusCode::AlreadyExists
            | GrpcStatusCode::FailedPrecondition
            | GrpcStatusCode::Aborted
            | GrpcStatusCode::OutOfRange
            | GrpcStatusCode::Unimplemented
            | GrpcStatusCode::DataLoss => RetryDecision::Terminal,
        },
        GrpcOutboundError::ConnectionFailed(_) => RetryDecision::Retry,
        GrpcOutboundError::Unavailable(_) => RetryDecision::Retry,
        GrpcOutboundError::Timeout(_)
        | GrpcOutboundError::Internal(_)
        | GrpcOutboundError::Cancelled(_) => RetryDecision::Terminal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── classify_resource_exhausted ──────────────────────────────────────────

    /// @covers: classify_resource_exhausted — rate-limit keywords → RateLimit.
    #[test]
    fn test_classify_resource_exhausted_rate_limit_keywords() {
        assert_eq!(
            classify_resource_exhausted("rate limit exceeded"),
            ResourceExhaustedContext::RateLimit
        );
        assert_eq!(
            classify_resource_exhausted("too many requests"),
            ResourceExhaustedContext::RateLimit
        );
        assert_eq!(
            classify_resource_exhausted("throttled"),
            ResourceExhaustedContext::RateLimit
        );
        assert_eq!(
            classify_resource_exhausted("RATE_LIMIT_ERROR"),
            ResourceExhaustedContext::RateLimit
        );
    }

    /// @covers: classify_resource_exhausted — quota keywords → HardQuota.
    #[test]
    fn test_classify_resource_exhausted_quota_keywords() {
        assert_eq!(
            classify_resource_exhausted("quota exceeded"),
            ResourceExhaustedContext::HardQuota
        );
        assert_eq!(
            classify_resource_exhausted("billing limit hit"),
            ResourceExhaustedContext::HardQuota
        );
        assert_eq!(
            classify_resource_exhausted("plan limit reached"),
            ResourceExhaustedContext::HardQuota
        );
    }

    /// @covers: classify_resource_exhausted — unknown message defaults to Capacity.
    #[test]
    fn test_classify_resource_exhausted_unknown_defaults_to_capacity() {
        assert_eq!(
            classify_resource_exhausted("out of memory"),
            ResourceExhaustedContext::Capacity
        );
        assert_eq!(
            classify_resource_exhausted("server overloaded"),
            ResourceExhaustedContext::Capacity
        );
        assert_eq!(
            classify_resource_exhausted(""),
            ResourceExhaustedContext::Capacity
        );
    }

    // ── parse_retry_after_hint ────────────────────────────────────────────────

    /// @covers: parse_retry_after_hint — extracts seconds from embedded tag.
    #[test]
    fn test_parse_retry_after_hint_extracts_seconds() {
        assert_eq!(
            parse_retry_after_hint("rate limit exceeded [retry-after=30s]"),
            Some(Duration::from_secs(30)),
        );
    }

    /// @covers: parse_retry_after_hint — returns None when tag absent.
    #[test]
    fn test_parse_retry_after_hint_returns_none_when_absent() {
        assert_eq!(parse_retry_after_hint("rate limit exceeded"), None);
        assert_eq!(parse_retry_after_hint(""), None);
    }

    // ── classify ─────────────────────────────────────────────────────────────

    /// @covers: classify — Unavailable status retries.
    #[test]
    fn test_classify_status_unavailable_returns_retry() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::Unavailable,
            "lb".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::Retry);
    }

    /// @covers: classify — ResourceExhausted(RateLimit) → RetryRateLimit.
    #[test]
    fn test_classify_resource_exhausted_rate_limit_returns_retry_rate_limit() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::ResourceExhausted,
            "rate limit exceeded".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::RetryRateLimit);
        assert!(classify(&r).is_rate_limit());
        assert!(classify(&r).should_retry());
    }

    /// @covers: classify — ResourceExhausted(Capacity) → Retry.
    #[test]
    fn test_classify_resource_exhausted_capacity_returns_retry() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::ResourceExhausted,
            "server overloaded".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::Retry);
        assert!(!classify(&r).is_rate_limit());
    }

    /// @covers: classify — ResourceExhausted(HardQuota) → Terminal.
    #[test]
    fn test_classify_resource_exhausted_hard_quota_is_terminal() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::ResourceExhausted,
            "quota exceeded".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::Terminal);
        assert!(!classify(&r).should_retry());
    }

    /// @covers: classify — PermissionDenied is NEVER retried.
    #[test]
    fn test_classify_status_permission_denied_is_terminal() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::PermissionDenied,
            "no".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::Terminal);
        assert!(!classify(&r).should_retry());
    }

    /// @covers: classify — Unauthenticated is NEVER retried.
    #[test]
    fn test_classify_status_unauthenticated_is_terminal() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::Unauthenticated,
            "bad token".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::Terminal);
        assert!(!classify(&r).should_retry());
    }

    /// @covers: classify — DeadlineExceeded is terminal.
    #[test]
    fn test_classify_status_deadline_exceeded_is_terminal() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::DeadlineExceeded,
            "tick".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::Terminal);
    }

    /// @covers: classify — Internal is terminal (server bug).
    #[test]
    fn test_classify_status_internal_is_terminal() {
        let r: Result<(), _> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::Internal,
            "bug".into(),
        ));
        assert_eq!(classify(&r), RetryDecision::Terminal);
    }

    /// @covers: classify — ConnectionFailed retries.
    #[test]
    fn test_classify_connection_failed_retries() {
        let r: Result<(), _> = Err(GrpcOutboundError::ConnectionFailed("rst".into()));
        assert_eq!(classify(&r), RetryDecision::Retry);
    }

    /// @covers: classify — Timeout is terminal.
    #[test]
    fn test_classify_timeout_is_terminal() {
        let r: Result<(), _> = Err(GrpcOutboundError::Timeout("deadline".into()));
        assert_eq!(classify(&r), RetryDecision::Terminal);
    }

    /// @covers: classify — Cancelled is terminal.
    #[test]
    fn test_classify_cancelled_is_terminal() {
        let r: Result<(), _> = Err(GrpcOutboundError::Cancelled("token".into()));
        assert_eq!(classify(&r), RetryDecision::Terminal);
    }

    /// @covers: classify — Ok is success, not retry.
    #[test]
    fn test_classify_ok_returns_success() {
        let r: Result<i32, GrpcOutboundError> = Ok(42);
        assert_eq!(classify(&r), RetryDecision::Success);
        assert!(!classify(&r).should_retry());
    }

    /// @covers: should_retry — Retry and RetryRateLimit are retryable.
    #[test]
    fn test_should_retry_true_for_retry_variants() {
        assert!(RetryDecision::Retry.should_retry());
        assert!(RetryDecision::RetryRateLimit.should_retry());
        assert!(!RetryDecision::Success.should_retry());
        assert!(!RetryDecision::Terminal.should_retry());
    }

    /// @covers: classify — every non-retry status code is terminal.
    #[test]
    fn test_classify_non_retry_status_codes_all_terminal() {
        for code in [
            GrpcStatusCode::Cancelled,
            GrpcStatusCode::Unknown,
            GrpcStatusCode::InvalidArgument,
            GrpcStatusCode::NotFound,
            GrpcStatusCode::AlreadyExists,
            GrpcStatusCode::FailedPrecondition,
            GrpcStatusCode::Aborted,
            GrpcStatusCode::OutOfRange,
            GrpcStatusCode::Unimplemented,
            GrpcStatusCode::DataLoss,
        ] {
            let r: Result<(), _> = Err(GrpcOutboundError::Status(code, "x".into()));
            assert_eq!(
                classify(&r),
                RetryDecision::Terminal,
                "expected {code:?} to be Terminal",
            );
        }
    }
}
