//! `GrpcChannelConfigBuilder` — builder for [`GrpcChannelConfig`].

use super::grpc_channel_config::{GrpcChannelConfig, DEFAULT_MAX_MESSAGE_BYTES};
use crate::api::value::compression_mode::CompressionMode;
use crate::api::value::keep_alive_config::KeepAliveConfig;
use crate::api::value::mtls_config::MtlsConfig;
use crate::api::value::resilience::resilience_config::ResilienceConfig;

/// Builder for [`GrpcChannelConfig`].
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct GrpcChannelConfigBuilder {
    endpoint: Option<String>,
    tls_required: bool,
    mtls: Option<MtlsConfig>,
    keep_alive: Option<KeepAliveConfig>,
    max_message_bytes: Option<usize>,
    compression: Option<CompressionMode>,
    resilience: Option<ResilienceConfig>,
}

#[allow(dead_code)]
impl GrpcChannelConfigBuilder {
    pub fn new() -> Self {
        Self {
            tls_required: true,
            ..Default::default()
        }
    }
    pub fn endpoint(mut self, v: impl Into<String>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
    pub fn allow_plaintext(mut self) -> Self {
        self.tls_required = false;
        self
    }
    pub fn mtls(mut self, v: MtlsConfig) -> Self {
        self.mtls = Some(v);
        self
    }
    pub fn keep_alive(mut self, v: KeepAliveConfig) -> Self {
        self.keep_alive = Some(v);
        self
    }
    pub fn max_message_bytes(mut self, v: usize) -> Self {
        self.max_message_bytes = Some(v);
        self
    }
    pub fn compression(mut self, v: CompressionMode) -> Self {
        self.compression = Some(v);
        self
    }
    pub fn resilience(mut self, v: ResilienceConfig) -> Self {
        self.resilience = Some(v);
        self
    }

    /// Build the [`GrpcChannelConfig`]. Returns `Err` when endpoint is unset.
    pub fn build(self) -> Result<GrpcChannelConfig, String> {
        let endpoint = self.endpoint.ok_or("endpoint required")?;
        let mut cfg = GrpcChannelConfig::new(endpoint);
        cfg.tls_required = self.tls_required;
        cfg.mtls = self.mtls;
        cfg.keep_alive = self.keep_alive;
        cfg.max_message_bytes = self.max_message_bytes.unwrap_or(DEFAULT_MAX_MESSAGE_BYTES);
        cfg.compression = self.compression.unwrap_or(CompressionMode::None);
        cfg.resilience = self.resilience;
        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: build
    /// @covers: endpoint
    #[test]
    fn test_build_with_endpoint_returns_ok() {
        let cfg = GrpcChannelConfigBuilder::new()
            .endpoint("https://example.com:443")
            .build();
        assert!(cfg.is_ok());
    }

    /// @covers: build
    #[test]
    fn test_build_without_endpoint_returns_err() {
        assert!(GrpcChannelConfigBuilder::new().build().is_err());
    }

    /// @covers: allow_plaintext
    #[test]
    fn test_allow_plaintext_disables_tls() {
        let cfg = GrpcChannelConfigBuilder::new()
            .endpoint("http://localhost:50051")
            .allow_plaintext()
            .build()
            .unwrap();
        assert!(!cfg.tls_required);
    }

    /// @covers: mtls
    #[test]
    fn test_mtls_stores_client_identity() {
        let m = MtlsConfig::new("cert.pem", "key.pem");
        let cfg = GrpcChannelConfigBuilder::new()
            .endpoint("https://example.com:443")
            .mtls(m)
            .build()
            .unwrap();
        assert!(cfg.mtls.is_some());
    }

    /// @covers: keep_alive
    #[test]
    fn test_keep_alive_stores_config() {
        use std::time::Duration;
        let ka = KeepAliveConfig {
            interval: Duration::from_secs(5),
            timeout: Duration::from_secs(10),
            permit_without_calls: true,
        };
        let cfg = GrpcChannelConfigBuilder::new()
            .endpoint("https://example.com:443")
            .keep_alive(ka)
            .build()
            .unwrap();
        assert!(cfg.keep_alive.is_some());
    }

    /// @covers: max_message_bytes
    #[test]
    fn test_max_message_bytes_overrides_default() {
        let cfg = GrpcChannelConfigBuilder::new()
            .endpoint("https://example.com:443")
            .max_message_bytes(8 * 1024 * 1024)
            .build()
            .unwrap();
        assert_eq!(cfg.max_message_bytes, 8 * 1024 * 1024);
    }

    /// @covers: compression
    #[test]
    fn test_compression_sets_mode() {
        let cfg = GrpcChannelConfigBuilder::new()
            .endpoint("https://example.com:443")
            .compression(CompressionMode::Gzip)
            .build()
            .unwrap();
        assert_eq!(cfg.compression, CompressionMode::Gzip);
    }

    /// @covers: resilience
    #[test]
    fn test_resilience_stores_policy() {
        let r = ResilienceConfig {
            max_attempts: 3,
            initial_backoff_ms: 100,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
            max_backoff_ms: 2_000,
            rate_limit_max_attempts: 2,
            rate_limit_initial_backoff_ms: 1_000,
            rate_limit_max_backoff_ms: 10_000,
            failure_threshold: 5,
            cool_down_seconds: 10,
            half_open_probe_count: 1,
        };
        let cfg = GrpcChannelConfigBuilder::new()
            .endpoint("https://example.com:443")
            .resilience(r)
            .build()
            .unwrap();
        assert!(cfg.resilience.is_some());
    }
}
