//! `RetryPolicyBuilder` — builder for [`RetryPolicy`].

use std::time::Duration;

use super::retry_policy::RetryPolicy;

/// Builder for [`RetryPolicy`].
pub(crate) struct RetryPolicyBuilder {
    pub max_attempts:               u32,
    pub initial_backoff:            Duration,
    pub backoff_multiplier:         f64,
    pub jitter_factor:              f64,
    pub max_backoff:                Duration,
    pub rate_limit_max_attempts:    u32,
    pub rate_limit_initial_backoff: Duration,
    pub rate_limit_max_backoff:     Duration,
}

impl RetryPolicyBuilder {
    pub(crate) fn build(self) -> RetryPolicy {
        RetryPolicy {
            max_attempts:               self.max_attempts,
            initial_backoff:            self.initial_backoff,
            backoff_multiplier:         self.backoff_multiplier,
            jitter_factor:              self.jitter_factor,
            max_backoff:                self.max_backoff,
            rate_limit_max_attempts:    self.rate_limit_max_attempts,
            rate_limit_initial_backoff: self.rate_limit_initial_backoff,
            rate_limit_max_backoff:     self.rate_limit_max_backoff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_constructs_retry_policy() {
        let p = RetryPolicyBuilder {
            max_attempts:               3,
            initial_backoff:            Duration::ZERO,
            backoff_multiplier:         1.0,
            jitter_factor:              0.0,
            max_backoff:                Duration::ZERO,
            rate_limit_max_attempts:    2,
            rate_limit_initial_backoff: Duration::ZERO,
            rate_limit_max_backoff:     Duration::ZERO,
        }.build();
        assert_eq!(p.max_attempts, 3);
    }
}
