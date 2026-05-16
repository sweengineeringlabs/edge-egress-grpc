//! Outbound channel configuration — TLS-by-default, fail-closed.

use serde::{Deserialize, Serialize};

use crate::api::value_object::compression_mode::CompressionMode;
use crate::api::value_object::keep_alive_config::KeepAliveConfig;
use crate::api::value_object::mtls_config::MtlsConfig;
use crate::api::value_object::resilience::resilience_config::ResilienceConfig;

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

#[cfg(test)]
mod tests {
    use super::*;

    /// Issue #5 acceptance gate.
    #[test]
    fn test_default_sets_tls_required_to_true() {
        let cfg = GrpcChannelConfig::default();
        assert!(cfg.tls_required, "TLS-by-default invariant broken");
    }

    #[test]
    fn test_default_max_message_bytes_is_four_mib() {
        let cfg = GrpcChannelConfig::default();
        assert_eq!(cfg.max_message_bytes, 4 * 1024 * 1024);
    }

    #[test]
    fn test_default_compression_is_none() {
        let cfg = GrpcChannelConfig::default();
        assert_eq!(cfg.compression, CompressionMode::None);
    }

    #[test]
    fn test_new_sets_endpoint_and_keeps_tls_required() {
        let cfg = GrpcChannelConfig::new("https://x.example.com:443");
        assert_eq!(cfg.endpoint, "https://x.example.com:443");
        assert!(cfg.tls_required);
    }

    /// @covers: allow_plaintext
    #[test]
    fn test_allow_plaintext_relaxes_tls_requirement() {
        let cfg = GrpcChannelConfig::new("http://localhost:50051").allow_plaintext();
        assert!(!cfg.tls_required);
    }

    /// @covers: with_mtls
    #[test]
    fn test_with_mtls_stores_identity() {
        let cfg = GrpcChannelConfig::new("https://x").with_mtls(MtlsConfig::new("c.pem", "k.pem"));
        let mtls = cfg.mtls.expect("mtls must be Some");
        assert_eq!(mtls.cert_pem_path, "c.pem");
    }

    /// @covers: with_compression
    #[test]
    fn test_with_compression_switches_mode() {
        let cfg = GrpcChannelConfig::new("https://x").with_compression(CompressionMode::Gzip);
        assert_eq!(cfg.compression, CompressionMode::Gzip);
    }

    /// @covers: with_keep_alive
    #[test]
    fn test_with_keep_alive_stores_config() {
        use std::time::Duration;
        let ka = KeepAliveConfig {
            interval: Duration::from_secs(5),
            timeout: Duration::from_secs(10),
            permit_without_calls: true,
        };
        let cfg = GrpcChannelConfig::new("https://x").with_keep_alive(ka);
        let stored = cfg.keep_alive.expect("keep_alive must be Some");
        assert_eq!(stored.interval, Duration::from_secs(5));
    }

    /// @covers: with_max_message_bytes
    #[test]
    fn test_with_max_message_bytes_overrides_default() {
        let cfg = GrpcChannelConfig::new("https://x").with_max_message_bytes(8 * 1024 * 1024);
        assert_eq!(cfg.max_message_bytes, 8 * 1024 * 1024);
    }

    /// @covers: with_request_timeout
    #[test]
    fn test_with_request_timeout_stores_seconds() {
        use std::time::Duration;
        let cfg = GrpcChannelConfig::new("https://x").with_request_timeout(Duration::from_secs(60));
        assert_eq!(cfg.request_timeout_secs, Some(60));
    }

    /// @covers: with_request_timeout
    #[test]
    fn test_with_request_timeout_clamps_sub_second_to_one_second() {
        use std::time::Duration;
        let cfg =
            GrpcChannelConfig::new("https://x").with_request_timeout(Duration::from_millis(500));
        assert_eq!(cfg.request_timeout_secs, Some(1));
    }

    /// @covers: with_request_timeout
    #[test]
    fn test_default_request_timeout_secs_is_none() {
        assert!(GrpcChannelConfig::new("https://x")
            .request_timeout_secs
            .is_none());
    }

    /// @covers: with_resilience
    #[test]
    fn test_with_resilience_stores_policy() {
        use crate::api::value_object::resilience::resilience_config::ResilienceConfig;
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
        let cfg = GrpcChannelConfig::new("https://x").with_resilience(r);
        assert!(cfg.resilience.is_some());
    }
}
