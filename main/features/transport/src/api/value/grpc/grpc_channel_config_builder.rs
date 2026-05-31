//! `GrpcChannelConfigBuilder` — builder for [`GrpcChannelConfig`].

use super::grpc_channel_config::{GrpcChannelConfig, DEFAULT_MAX_MESSAGE_BYTES};
use crate::api::value::compression_mode::CompressionMode;
use crate::api::value::keep_alive_config::KeepAliveConfig;
use crate::api::value::mtls_config::MtlsConfig;
use crate::api::value::resilience::resilience_config::ResilienceConfig;

/// Builder for [`GrpcChannelConfig`].
#[derive(Debug, Default)]
pub struct GrpcChannelConfigBuilder {
    endpoint: Option<String>,
    tls_required: bool,
    mtls: Option<MtlsConfig>,
    keep_alive: Option<KeepAliveConfig>,
    max_message_bytes: Option<usize>,
    compression: Option<CompressionMode>,
    resilience: Option<ResilienceConfig>,
}

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
