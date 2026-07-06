//! `impl` blocks for [`GrpcChannelConfig`]. The type *declaration* lives in `api/`.

use crate::api::{CompressionMode, GrpcChannelConfig, DEFAULT_MAX_MESSAGE_BYTES};
use crate::api::{KeepAliveConfig, MtlsConfig, ResilienceConfig};

impl GrpcChannelConfig {
    /// Construct a TLS-required channel for `endpoint`.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            tls_required: true,
            mtls: None,
            keep_alive: None,
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            compression: CompressionMode::None,
            resilience: None,
            request_timeout_secs: None,
        }
    }

    /// Explicitly relax the TLS requirement.
    pub fn allow_plaintext(mut self) -> Self {
        self.tls_required = false;
        self
    }

    /// Attach an mTLS client identity.
    pub fn with_mtls(mut self, mtls: MtlsConfig) -> Self {
        self.mtls = Some(mtls);
        self
    }

    /// Attach an HTTP/2 keep-alive policy.
    pub fn with_keep_alive(mut self, keep_alive: KeepAliveConfig) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    /// Set the max-message-bytes ceiling.
    pub fn with_max_message_bytes(mut self, bytes: usize) -> Self {
        self.max_message_bytes = bytes;
        self
    }

    /// Set the compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    /// Attach a resilience policy (retry + circuit breaker).
    pub fn with_resilience(mut self, policy: ResilienceConfig) -> Self {
        self.resilience = Some(policy);
        self
    }

    /// Set the client-side fallback request timeout.
    ///
    /// This backstop is applied on every request via `tokio::time::timeout`,
    /// independent of the per-call `GrpcRequest::deadline` header. Use it to
    /// defend against unresponsive upstreams when your call deadlines vary or
    /// are not set.
    pub fn with_request_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request_timeout_secs = Some(timeout.as_secs().max(1));
        self
    }
}

impl Default for GrpcChannelConfig {
    /// Defaults: empty endpoint, TLS required, no mTLS, no keep-alive,
    /// 4 MiB message cap, no compression.
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            tls_required: true,
            mtls: None,
            keep_alive: None,
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            compression: CompressionMode::None,
            resilience: None,
            request_timeout_secs: None,
        }
    }
}
