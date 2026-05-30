//! gRPC circuit-breaker policy schema.
//!
//! Policy **values** live in TOML:
//! - crate-shipped baseline: `config/application.toml`
//! - consumer override: whatever TOML the binary loads and
//!   passes to [`GrpcBreakerConfig::from_config`].

use std::time::Duration;

use serde::Deserialize;

use crate::api::breaker::error::Error;

/// gRPC circuit-breaker policy schema.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GrpcBreakerConfig {
    /// Consecutive failures that trip the breaker open.
    pub failure_threshold: u32,

    /// Seconds to wait in Open state before the next request
    /// promotes to HalfOpen.
    pub cool_down_seconds: u64,

    /// Consecutive probe successes required in HalfOpen to
    /// close the breaker.
    pub half_open_probe_count: u32,
}

impl Default for GrpcBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            cool_down_seconds: 30,
            half_open_probe_count: 1,
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for GrpcBreakerConfig {
    fn section_name() -> &'static str {
        "grpc_breaker"
    }
}

impl GrpcBreakerConfig {
    /// Parse a config from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, Error> {
        let cfg: Self = toml::from_str(toml_text).map_err(|e| Error::ParseFailed(e.to_string()))?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Cool-down as a [`Duration`].
    pub fn cool_down(&self) -> Duration {
        Duration::from_secs(self.cool_down_seconds)
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.failure_threshold == 0 {
            return Err(Error::InvalidConfig(
                "failure_threshold must be >= 1".into(),
            ));
        }
        if self.half_open_probe_count == 0 {
            return Err(Error::InvalidConfig(
                "half_open_probe_count must be >= 1".into(),
            ));
        }
        Ok(())
    }
}
