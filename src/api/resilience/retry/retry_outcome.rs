//! `RetryOutcome` — public representation of a retry policy decision.

use std::time::Duration;

/// A retry decision returned by a policy.
pub enum RetryOutcome {
    /// Retry after sleeping the given duration.
    Retry(Duration),
    /// Do not retry — propagate the error.
    DoNotRetry,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_outcome_do_not_retry_variant_is_constructable() {
        let _ = RetryOutcome::DoNotRetry;
    }

    #[test]
    fn test_retry_outcome_retry_variant_carries_duration() {
        let d = RetryOutcome::Retry(Duration::from_millis(50));
        match d {
            RetryOutcome::Retry(dur) => assert_eq!(dur.as_millis(), 50),
            RetryOutcome::DoNotRetry => panic!("expected Retry"),
        }
    }
}
