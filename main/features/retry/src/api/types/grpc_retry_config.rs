//! gRPC retry policy schema.
//!
//! Policy **values** live in TOML:
//! - crate-shipped baseline: `config/application.toml`
//! - consumer override: whatever TOML the binary loads and
//!   passes to [`GrpcRetryConfig::from_config`].
//!
//! No `Default` impl — per the config-driven principle, policy
//! is data in a file, not literals in a source tree.

use std::time::Duration;

use serde::Deserialize;

use crate::api::error::Error;

/// gRPC retry policy schema.  Construct via
/// [`GrpcRetryConfig::from_config`] (custom TOML) or
/// [`GrpcRetryConfig::swe_default`] (crate baseline).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrpcRetryConfig {
    /// Maximum attempts per request (1 = no retry).
    pub max_attempts: u32,

    /// Delay before the first retry, in milliseconds.
    pub initial_backoff_ms: u64,

    /// Exponential backoff base (e.g. 2.0 → 100ms, 200ms, 400ms).
    pub backoff_multiplier: f64,

    /// Jitter as a fraction of the computed backoff (0.0 = none,
    /// 0.1 = up to 10% random delta).
    pub jitter_factor: f64,

    /// Upper bound on any single retry backoff in milliseconds.
    pub max_backoff_ms: u64,

    // ── Rate-limit track ─────────────────────────────────────────────────────
    // Used when `classify()` returns `RetryRateLimit` (RESOURCE_EXHAUSTED
    // with a rate-limit message).  The upstream reset window is typically
    // seconds to minutes, so the floor is much higher than the standard
    // track.  The Retry-After hint from the server overrides the computed
    // backoff when present.
    /// Max attempts specifically for rate-limit `RESOURCE_EXHAUSTED`.
    /// Often lower than `max_attempts` — rate-limit retries are expensive.
    pub rate_limit_max_attempts: u32,

    /// Initial backoff for rate-limit errors, in milliseconds.
    /// Overridden by a `[retry-after=Ns]` hint when present.
    pub rate_limit_initial_backoff_ms: u64,

    /// Upper bound on the rate-limit backoff, in milliseconds.
    pub rate_limit_max_backoff_ms: u64,
}

impl Default for GrpcRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff_ms: 100,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            max_backoff_ms: 5000,
            rate_limit_max_attempts: 2,
            rate_limit_initial_backoff_ms: 1000,
            rate_limit_max_backoff_ms: 10000,
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for GrpcRetryConfig {
    fn section_name() -> &'static str {
        "grpc_retry"
    }
}

impl GrpcRetryConfig {
    /// Parse a config from TOML text.
    ///
    /// Returns [`Error::ParseFailed`] when the text isn't valid
    /// TOML, when a required key is missing, or when an unknown
    /// key is present (`deny_unknown_fields` is set).  Returns
    /// [`Error::InvalidConfig`] when a value is out of range
    /// (e.g. `backoff_multiplier <= 0.0`).
    pub fn from_config(toml_text: &str) -> Result<Self, Error> {
        let cfg: Self = toml::from_str(toml_text).map_err(|e| Error::ParseFailed(e.to_string()))?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Initial backoff as a [`Duration`].
    pub fn initial_backoff(&self) -> Duration {
        Duration::from_millis(self.initial_backoff_ms)
    }

    /// Maximum single-retry backoff as a [`Duration`].
    pub fn max_backoff(&self) -> Duration {
        Duration::from_millis(self.max_backoff_ms)
    }

    /// Initial rate-limit backoff as a [`Duration`].
    pub fn rate_limit_initial_backoff(&self) -> Duration {
        Duration::from_millis(self.rate_limit_initial_backoff_ms)
    }

    /// Maximum rate-limit backoff as a [`Duration`].
    pub fn rate_limit_max_backoff(&self) -> Duration {
        Duration::from_millis(self.rate_limit_max_backoff_ms)
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.max_attempts == 0 {
            return Err(Error::InvalidConfig("max_attempts must be >= 1".into()));
        }
        if self.backoff_multiplier <= 0.0 || !self.backoff_multiplier.is_finite() {
            return Err(Error::InvalidConfig(
                "backoff_multiplier must be a positive finite number".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.jitter_factor) || !self.jitter_factor.is_finite() {
            return Err(Error::InvalidConfig(
                "jitter_factor must be in [0.0, 1.0]".into(),
            ));
        }
        if self.max_backoff_ms < self.initial_backoff_ms {
            return Err(Error::InvalidConfig(
                "max_backoff_ms must be >= initial_backoff_ms".into(),
            ));
        }
        if self.rate_limit_max_attempts == 0 {
            return Err(Error::InvalidConfig(
                "rate_limit_max_attempts must be >= 1".into(),
            ));
        }
        if self.rate_limit_max_backoff_ms < self.rate_limit_initial_backoff_ms {
            return Err(Error::InvalidConfig(
                "rate_limit_max_backoff_ms must be >= rate_limit_initial_backoff_ms".into(),
            ));
        }
        Ok(())
    }
}
