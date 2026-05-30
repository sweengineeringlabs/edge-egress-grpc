//! `TonicGrpcClient` struct declaration, type alias, and constructor.

use std::time::Duration;

use bytes::Bytes;
use http_body_util::Full;
use hyper_util::client::legacy::connect::HttpConnector;

use crate::api::interceptor::GrpcEgressInterceptorChain;
use crate::api::value::{CompressionMode, DEFAULT_MAX_MESSAGE_BYTES};

/// Hyper HTTP/2 client type alias used by [`TonicGrpcClient`].
pub(crate) type HyperClient =
    hyper_util::client::legacy::Client<hyper_rustls::HttpsConnector<HttpConnector>, Full<Bytes>>;

/// Concrete `GrpcEgress` implementation using hyper HTTP/2.
pub struct TonicGrpcClient {
    pub(crate) base_uri: String,
    pub(crate) client: HyperClient,
    pub(crate) timeout: Duration,
    pub(crate) interceptors: GrpcEgressInterceptorChain,
    pub(crate) max_message_bytes: usize,
    pub(crate) compression: CompressionMode,
}

impl TonicGrpcClient {
    /// Create a client with a 30-second fallback timeout.
    pub fn new(base_uri: impl Into<String>) -> Self {
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http2()
            .build();
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .http2_only(true)
                .build(connector);
        Self {
            base_uri: base_uri.into(),
            client,
            timeout: Duration::from_secs(30),
            interceptors: GrpcEgressInterceptorChain::new(),
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            compression: CompressionMode::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ensure_rustls_provider() {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        });
    }

    #[test]
    fn test_new_creates_client_with_given_base_uri() {
        ensure_rustls_provider();
        let c = TonicGrpcClient::new("http://127.0.0.1:50051");
        assert_eq!(c.base_uri, "http://127.0.0.1:50051");
        assert_eq!(c.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_tonic_grpc_client_struct_is_accessible() {
        ensure_rustls_provider();
        let c = TonicGrpcClient::new("http://127.0.0.1:50051");
        let _ = std::mem::size_of_val(&c);
    }
}
