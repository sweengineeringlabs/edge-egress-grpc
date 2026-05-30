//! `TonicGrpcClientBuilder` — public API builder for [`crate::api::client::tonic_grpc_client::TonicGrpcClient`].
//!
//! Callers use this to construct a [`crate::api::client::tonic_grpc_client::TonicGrpcClient`] from individual
//! settings rather than a full [`crate::api::value::GrpcChannelConfig`].
//! For config-driven construction prefer
//! [`crate::create_tonic_client_from_config`].

use std::time::Duration;

use crate::api::interceptor::GrpcEgressInterceptorChain;
use crate::api::value::{CompressionMode, DEFAULT_MAX_MESSAGE_BYTES};

/// Builder for [`crate::api::client::tonic_grpc_client::TonicGrpcClient`].
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

    /// Consume the builder and return a configured [`crate::api::client::tonic_grpc_client::TonicGrpcClient`].
    pub fn build(self) -> crate::api::client::tonic_grpc_client::TonicGrpcClient {
        crate::api::client::tonic_grpc_client::TonicGrpcClient::new(self.base_uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_rustls_provider() {
        use std::sync::Once;
        static ONCE: Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        });
    }

    #[test]
    fn test_new_creates_builder_with_30s_default_timeout() {
        ensure_rustls_provider();
        let b = TonicGrpcClientBuilder::new("http://localhost:50051");
        assert_eq!(b.timeout, Duration::from_secs(30));
    }

    /// @covers: timeout
    #[test]
    fn test_timeout_overrides_default() {
        ensure_rustls_provider();
        let b =
            TonicGrpcClientBuilder::new("http://localhost:50051").timeout(Duration::from_secs(5));
        assert_eq!(b.timeout, Duration::from_secs(5));
    }

    /// @covers: max_message_bytes
    #[test]
    fn test_max_message_bytes_overrides_default() {
        ensure_rustls_provider();
        let b = TonicGrpcClientBuilder::new("http://localhost:50051")
            .max_message_bytes(8 * 1024 * 1024);
        assert_eq!(b.max_message_bytes, 8 * 1024 * 1024);
    }

    /// @covers: compression
    #[test]
    fn test_compression_overrides_default() {
        ensure_rustls_provider();
        let b = TonicGrpcClientBuilder::new("http://localhost:50051")
            .compression(CompressionMode::Gzip);
        assert!(matches!(b.compression, CompressionMode::Gzip));
    }

    /// @covers: build
    #[test]
    fn test_build_produces_client() {
        ensure_rustls_provider();
        let client = TonicGrpcClientBuilder::new("http://localhost:50051").build();
        assert_eq!(client.base_uri, "http://localhost:50051");
    }

    /// @covers: interceptors
    #[test]
    fn test_interceptors_overrides_default() {
        ensure_rustls_provider();
        let chain = GrpcEgressInterceptorChain::new();
        let _ = TonicGrpcClientBuilder::new("http://localhost:50051")
            .interceptors(chain)
            .build();
    }
}
