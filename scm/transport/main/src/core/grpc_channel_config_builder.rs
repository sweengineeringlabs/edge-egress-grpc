//! `impl` block for [`GrpcChannelConfigBuilder`]. The type *declaration* lives in `api/`.

use crate::api::{CompressionMode, GrpcChannelConfig, GrpcChannelConfigBuilder};
use crate::api::{KeepAliveConfig, MtlsConfig, ResilienceConfig, DEFAULT_MAX_MESSAGE_BYTES};

impl GrpcChannelConfigBuilder {
    /// Create a new builder with TLS required by default.
    pub fn new() -> Self {
        Self {
            tls_required: true,
            ..Default::default()
        }
    }
    /// Set the gRPC endpoint URI.
    pub fn endpoint(mut self, v: impl Into<String>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
    /// Allow plaintext (non-TLS) connections.
    pub fn allow_plaintext(mut self) -> Self {
        self.tls_required = false;
        self
    }
    /// Configure mutual TLS.
    pub fn mtls(mut self, v: MtlsConfig) -> Self {
        self.mtls = Some(v);
        self
    }
    /// Configure HTTP/2 keep-alive settings.
    pub fn keep_alive(mut self, v: KeepAliveConfig) -> Self {
        self.keep_alive = Some(v);
        self
    }
    /// Set the maximum allowed message size in bytes.
    pub fn max_message_bytes(mut self, v: usize) -> Self {
        self.max_message_bytes = Some(v);
        self
    }
    /// Set the compression mode for outbound messages.
    pub fn compression(mut self, v: CompressionMode) -> Self {
        self.compression = Some(v);
        self
    }
    /// Set the resilience (retry/circuit-breaker) configuration.
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
