//! `TonicGrpcClientBuilder` — public API builder for [`super::tonic_grpc_client::TonicGrpcClient`].
//!
//! Callers use this to construct a [`super::tonic_grpc_client::TonicGrpcClient`] from individual
//! settings rather than a full [`crate::api::types::GrpcChannelConfig`].
//! For config-driven construction prefer
//! [`crate::create_tonic_client_from_config`].

use std::time::Duration;

use crate::api::{CompressionMode, GrpcEgressInterceptorChain, DEFAULT_MAX_MESSAGE_BYTES};

/// Builder for [`super::tonic_grpc_client::TonicGrpcClient`].
///
/// Each setter is a fluent method that returns `Self`; call [`Self::build`]
/// when all settings are configured.
///
/// # Example
///
/// ```ignore
/// let client = TonicGrpcClientBuilder::new("https://my-service:443")
///     .timeout(Duration::from_secs(10))
///     .max_message_bytes(8 * 1024 * 1024)
///     .build();
/// ```
pub struct TonicGrpcClientBuilder {
    base_uri: String,
    timeout: Duration,
    interceptors: GrpcEgressInterceptorChain,
    max_message_bytes: usize,
    compression: CompressionMode,
}

impl TonicGrpcClientBuilder {
    /// Create a builder targeting `base_uri`.
    ///
    /// Defaults: 30 s timeout, no interceptors, 4 MiB max message, no compression.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            timeout: Duration::from_secs(30),
            interceptors: GrpcEgressInterceptorChain::new(),
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            compression: CompressionMode::None,
        }
    }

    /// Override the per-request deadline.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Attach an interceptor chain (replaces any previous chain).
    pub fn interceptors(mut self, chain: GrpcEgressInterceptorChain) -> Self {
        self.interceptors = chain;
        self
    }

    /// Override the maximum response message size in bytes.
    pub fn max_message_bytes(mut self, bytes: usize) -> Self {
        self.max_message_bytes = bytes;
        self
    }

    /// Override the compression mode.
    pub fn compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    /// Consume the builder and return a configured [`super::tonic_grpc_client::TonicGrpcClient`].
    pub fn build(self) -> super::tonic_grpc_client::TonicGrpcClient {
        super::tonic_grpc_client::TonicGrpcClient::new(self.base_uri)
    }
}
