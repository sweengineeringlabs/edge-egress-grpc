//! `RetryDecision` — outcome of one retry policy evaluation.

use std::time::Duration;

/// What the retry policy decided for one error on one attempt.
#[derive(Debug)]
pub(crate) enum RetryDecision {
    /// Retry after sleeping for the given duration.
    Retry(Duration),
    /// Propagate the error immediately — do not retry.
    DoNotRetry,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_not_retry_variant_is_debug() {
        let d = RetryDecision::DoNotRetry;
        assert!(format!("{d:?}").contains("DoNotRetry"));
    }

    #[test]
    fn test_retry_variant_carries_duration() {
        let d = RetryDecision::Retry(Duration::from_millis(50));
        match d {
            RetryDecision::Retry(dur) => assert_eq!(dur.as_millis(), 50),
            RetryDecision::DoNotRetry => panic!("expected Retry"),
        }
    }
}
