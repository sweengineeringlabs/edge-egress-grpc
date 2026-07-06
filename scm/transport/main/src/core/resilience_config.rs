//! `impl` block for [`ResilienceConfig`]. The type *declaration* lives in `api/`.

use crate::api::ResilienceConfig;

impl Default for ResilienceConfig {
    /// Returns the fast-stateless-gRPC profile: the calibrated baseline that
    /// confirmed ≤ 1.5× retry amplification in load tests.
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff_ms: 100,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            max_backoff_ms: 5_000,
            rate_limit_max_attempts: 2,
            rate_limit_initial_backoff_ms: 1_000,
            rate_limit_max_backoff_ms: 10_000,
            failure_threshold: 5,
            cool_down_seconds: 30,
            half_open_probe_count: 1,
        }
    }
}
