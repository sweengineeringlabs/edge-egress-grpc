//! Outbound channel configuration — TLS-by-default, fail-closed.

use serde::{Deserialize, Serialize};

use crate::api::value::compression_mode::CompressionMode;
use crate::api::value::keep_alive_config::KeepAliveConfig;
use crate::api::value::mtls_config::MtlsConfig;
use crate::api::value::resilience::resilience_config::ResilienceConfig;

/// Default ceiling for inbound message bytes (4 MiB).
pub const DEFAULT_MAX_MESSAGE_BYTES: usize = 4 * 1024 * 1024;

/// Default client-side fallback timeout in seconds.
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

/// Configuration for a single outbound gRPC channel.
///
/// **TLS-by-default**.  `tls_required` is `true` in
/// `Default::default()`.  Plaintext requires explicit
/// [`GrpcChannelConfig::allow_plaintext`] — fail-closed by design.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcChannelConfig {
    /// Channel endpoint.
    pub endpoint: String,
    /// Require TLS on the wire.  When `true` (default) and the
    /// endpoint is plaintext, the transport refuses to dial.
    pub tls_required: bool,
    /// Optional mTLS client identity.
    pub mtls: Option<MtlsConfig>,
    /// Optional HTTP/2 keep-alive policy.
    pub keep_alive: Option<KeepAliveConfig>,
    /// Hard cap on a single response message in bytes.
    pub max_message_bytes: usize,
    /// Compression mode for outbound payloads.
    pub compression: CompressionMode,
    /// Optional retry + circuit breaker policy.
    ///
    /// When `Some`, [`crate::saf::create_transport_from_config`] wraps the
    /// bare transport in a [`crate::ResilientGrpcClient`] (retry + circuit
    /// breaker). When `None`, the transport is returned unwrapped.
    #[serde(default)]
    pub resilience: Option<ResilienceConfig>,
    /// Client-side fallback timeout in seconds.
    ///
    /// Applied as a `tokio::time::timeout` backstop on each request, independent
    /// of the per-call `GrpcRequest::deadline` (which propagates as `grpc-timeout`
    /// and is enforced server-side).  When absent, defaults to
    /// [`DEFAULT_REQUEST_TIMEOUT_SECS`] (30 s).
    ///
    /// In TOML: `request_timeout_secs = 60`
    #[serde(default)]
    pub request_timeout_secs: Option<u64>,
}

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
