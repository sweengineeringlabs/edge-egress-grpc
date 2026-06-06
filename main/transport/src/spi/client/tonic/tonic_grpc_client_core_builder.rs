//! `TonicGrpcClientBuilder` — builder for [`TonicGrpcClient`].

use std::time::Duration;

use super::tonic_grpc_client::TonicGrpcClient;
use crate::api::types::interceptor::GrpcEgressInterceptorChain;
use crate::api::vo::{CompressionMode, DEFAULT_MAX_MESSAGE_BYTES};

/// Builder for [`TonicGrpcClient`].
pub(crate) struct TonicGrpcClientBuilder {
    base_uri: String,
    timeout: Duration,
    interceptors: GrpcEgressInterceptorChain,
    max_message_bytes: usize,
    compression: CompressionMode,
}

impl TonicGrpcClientBuilder {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only exercised in tests; production wiring pending"
        )
    )]
    pub(crate) fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            timeout: Duration::from_secs(30),
            interceptors: GrpcEgressInterceptorChain::new(),
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            compression: CompressionMode::None,
        }
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only exercised in tests; production wiring pending"
        )
    )]
    pub(crate) fn timeout(mut self, v: Duration) -> Self {
        self.timeout = v;
        self
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only exercised in tests; production wiring pending"
        )
    )]
    pub(crate) fn interceptors(mut self, v: GrpcEgressInterceptorChain) -> Self {
        self.interceptors = v;
        self
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only exercised in tests; production wiring pending"
        )
    )]
    pub(crate) fn max_message_bytes(mut self, v: usize) -> Self {
        self.max_message_bytes = v;
        self
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only exercised in tests; production wiring pending"
        )
    )]
    pub(crate) fn compression(mut self, v: CompressionMode) -> Self {
        self.compression = v;
        self
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "only exercised in tests; production wiring pending"
        )
    )]
    pub(crate) fn build(self) -> TonicGrpcClient {
        TonicGrpcClient::with_timeout(self.base_uri, self.timeout)
            .with_interceptors(self.interceptors)
            .with_max_message_bytes(self.max_message_bytes)
            .with_compression(self.compression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_constructs_with_defaults() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let b = TonicGrpcClientBuilder::new("http://localhost:50051");
        let _ = b.build();
    }

    #[test]
    fn test_build_applies_all_settings() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let _ = TonicGrpcClientBuilder::new("http://localhost:50051")
            .timeout(Duration::from_secs(10))
            .max_message_bytes(8 * 1024 * 1024)
            .compression(CompressionMode::Gzip)
            .build();
    }

    #[test]
    fn test_timeout_is_applied_in_build() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let _ = TonicGrpcClientBuilder::new("http://localhost:50051")
            .timeout(Duration::from_secs(5))
            .build();
    }

    #[test]
    fn test_interceptors_is_applied_in_build() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let chain = GrpcEgressInterceptorChain::new();
        let _ = TonicGrpcClientBuilder::new("http://localhost:50051")
            .interceptors(chain)
            .build();
    }

    #[test]
    fn test_max_message_bytes_is_applied_in_build() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let _ = TonicGrpcClientBuilder::new("http://localhost:50051")
            .max_message_bytes(2 * 1024 * 1024)
            .build();
    }

    #[test]
    fn test_compression_is_applied_in_build() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let _ = TonicGrpcClientBuilder::new("http://localhost:50051")
            .compression(CompressionMode::None)
            .build();
    }
}
