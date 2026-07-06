//! `ResilienceConfigBuilder` — builder for [`crate::api::ResilienceConfig`].

/// Builder for [`crate::api::ResilienceConfig`].
#[derive(Debug, Default)]
pub struct ResilienceConfigBuilder {
    pub(crate) max_attempts: Option<u32>,
    pub(crate) initial_backoff_ms: Option<u64>,
    pub(crate) backoff_multiplier: Option<f64>,
    pub(crate) jitter_factor: Option<f64>,
    pub(crate) max_backoff_ms: Option<u64>,
    pub(crate) rate_limit_max_attempts: Option<u32>,
    pub(crate) rate_limit_initial_backoff_ms: Option<u64>,
    pub(crate) rate_limit_max_backoff_ms: Option<u64>,
    pub(crate) failure_threshold: Option<u32>,
    pub(crate) cool_down_seconds: Option<u64>,
    pub(crate) half_open_probe_count: Option<u32>,
}
