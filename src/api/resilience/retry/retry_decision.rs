//! `RetryDecision` — public interface type for retry decision outcomes.

use std::time::Duration;

/// The public representation of what a retry policy decides for one error.
///
/// Maps 1:1 with the core-internal `RetryDecision` enum; this type exposes
/// the interface without leaking implementation details.
pub enum RetryDecision {
    /// Retry after sleeping the given duration.
    Retry(Duration),
    /// Propagate the error immediately — do not retry.
    DoNotRetry,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_decision_do_not_retry_variant_is_constructable() {
        let _ = RetryDecision::DoNotRetry;
    }

    #[test]
    fn test_retry_decision_retry_variant_carries_duration() {
        let d = RetryDecision::Retry(Duration::from_millis(50));
        match d {
            RetryDecision::Retry(dur) => assert_eq!(dur.as_millis(), 50),
            RetryDecision::DoNotRetry => panic!("expected Retry"),
        }
    }
}
