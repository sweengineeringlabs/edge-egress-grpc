//! Discrimination of `RESOURCE_EXHAUSTED` (gRPC 8) cause.
//!
//! Three distinct situations share the same status code but require
//! different retry strategies:
//!
//! | Context | Cause | Correct response |
//! |---------|-------|-----------------|
//! | `Capacity` | Server OOM / CPU saturation | Retry standard track |
//! | `RateLimit` | API rate-limit window full | Retry rate-limit track |
//! | `HardQuota` | Billing quota exhausted | Do not retry; escalate |

use std::time::Duration;

/// Discriminates the cause of a `RESOURCE_EXHAUSTED` (gRPC 8) error.
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
///
/// `Capacity` is the safe default — it triggers a retry which is better
/// than silently dropping the request if classification is wrong.
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
/// The transport embeds the HTTP `Retry-After` header value as
/// `[retry-after=Ns]` at the end of the `grpc-message` string when it
/// receives a `RESOURCE_EXHAUSTED` response with that header.
///
/// Returns `None` when no hint is present or the value cannot be parsed.
pub fn parse_retry_after_hint(message: &str) -> Option<Duration> {
    let tag   = "[retry-after=";
    let start = message.find(tag)? + tag.len();
    let rest  = &message[start..];
    let end   = rest.find('s')?;
    let secs: u64 = rest[..end].parse().ok()?;
    Some(Duration::from_secs(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: classify_resource_exhausted
    #[test]
    fn test_classify_resource_exhausted_rate_limit_keywords() {
        assert_eq!(classify_resource_exhausted("rate limit exceeded"), ResourceExhaustedContext::RateLimit);
        assert_eq!(classify_resource_exhausted("too many requests"),   ResourceExhaustedContext::RateLimit);
        assert_eq!(classify_resource_exhausted("throttled"),           ResourceExhaustedContext::RateLimit);
    }

    /// @covers: classify_resource_exhausted
    #[test]
    fn test_classify_resource_exhausted_quota_keywords() {
        assert_eq!(classify_resource_exhausted("quota exceeded"),    ResourceExhaustedContext::HardQuota);
        assert_eq!(classify_resource_exhausted("billing limit hit"), ResourceExhaustedContext::HardQuota);
    }

    /// @covers: classify_resource_exhausted
    #[test]
    fn test_classify_resource_exhausted_unknown_defaults_to_capacity() {
        assert_eq!(classify_resource_exhausted(""),             ResourceExhaustedContext::Capacity);
        assert_eq!(classify_resource_exhausted("server busy"), ResourceExhaustedContext::Capacity);
    }

    /// @covers: parse_retry_after_hint
    #[test]
    fn test_parse_retry_after_hint_extracts_seconds() {
        assert_eq!(
            parse_retry_after_hint("rate limit [retry-after=30s]"),
            Some(Duration::from_secs(30)),
        );
    }

    /// @covers: parse_retry_after_hint
    #[test]
    fn test_parse_retry_after_hint_absent_returns_none() {
        assert_eq!(parse_retry_after_hint("rate limit exceeded"), None);
        assert_eq!(parse_retry_after_hint(""), None);
    }
}
