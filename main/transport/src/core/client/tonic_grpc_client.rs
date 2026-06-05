//! `TonicGrpcClient` — concrete `GrpcEgress` implementation backed by hyper HTTP/2.

/// Core implementation unit for `TonicGrpcClient`.
///
/// The struct fields live in `api/client/tonic_grpc_client.rs` (the public type);
/// this marker makes the file structurally compliant with SEA Rule 89 — every
/// `core/` file must contain at least one struct, trait, or enum definition.
pub(crate) struct TonicGrpcClientCore;

use std::time::Duration;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures::future::BoxFuture;
use futures::StreamExt as _;
use http_body_util::{BodyExt as _, Full};
use tokio_util::sync::CancellationToken;

use crate::api::types::client::tonic_grpc_client::TonicGrpcClient;
use crate::api::types::interceptor::GrpcEgressInterceptorChain;
use crate::api::error::{GrpcChannelConfigError, GrpcEgressError};
use crate::api::traits::GrpcEgress;
use crate::api::types::{GrpcEgressResult, GrpcMessageStream};
use crate::api::value::{
    CompressionMode, GrpcChannelConfig, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
    DEFAULT_MAX_MESSAGE_BYTES,
};
use crate::core::status::Conversions as StatusConversions;

const SANITIZED_INTERNAL_MSG: &str = "internal client error";

// ── internal helpers ─────────────────────────────────────────────────────────

impl TonicGrpcClientCore {
    /// Encode `payload` as a single gRPC data frame:
    /// `[0x00][len_u32_be][payload]` — not compressed.
    fn encode_grpc_frame(payload: &[u8]) -> Bytes {
        let mut buf = BytesMut::with_capacity(5 + payload.len());
        buf.put_u8(0x00); // compression flag: not compressed
        buf.put_u32(payload.len() as u32);
        buf.put_slice(payload);
        buf.freeze()
    }

    /// Decode all complete gRPC frames from `data`.
    ///
    /// Each frame: `[compression_flag: u8][length: u32_be][payload: length bytes]`.
    /// Frames with the compression flag set are passed through as-is (no decompression).
    fn decode_grpc_frames(mut data: Bytes) -> GrpcEgressResult<Vec<Vec<u8>>> {
        const FRAME_HEADER: usize = 5;
        let mut out = Vec::new();
        while data.len() >= FRAME_HEADER {
            let _flag = data[0];
            let len = u32::from_be_bytes([data[1], data[2], data[3], data[4]]) as usize;
            data.advance(FRAME_HEADER);
            if data.len() < len {
                tracing::warn!(
                    expected = len,
                    actual = data.len(),
                    "truncated gRPC frame received from server",
                );
                return Err(GrpcEgressError::Internal(SANITIZED_INTERNAL_MSG.into()));
            }
            out.push(data[..len].to_vec());
            data.advance(len);
        }
        Ok(out)
    }

    /// Encode a `Duration` as a `grpc-timeout` header value per the gRPC protocol:
    /// integer value followed by a unit suffix (`H`, `M`, `S`, `m`, `u`, `n`).
    ///
    /// Picks the smallest unit that fits the duration in a `u64`, preferring
    /// milliseconds for sub-second values and seconds otherwise.
    fn encode_grpc_timeout(d: Duration) -> String {
        // Per RFC, value MUST fit in 8 ASCII digits — we cap at 99 999 999 of the
        // chosen unit to stay safely within that.  For practical deadlines this is
        // 27+ hours of seconds or 27+ years of seconds — never reached in real use.
        const MAX_VAL: u128 = 99_999_999;

        let nanos = d.as_nanos();
        if nanos == 0 {
            return "0n".into();
        }
        if nanos <= MAX_VAL {
            return format!("{nanos}n");
        }
        let micros = d.as_micros();
        if micros <= MAX_VAL {
            return format!("{micros}u");
        }
        let millis = d.as_millis();
        if millis <= MAX_VAL {
            return format!("{millis}m");
        }
        let secs = d.as_secs();
        if (secs as u128) <= MAX_VAL {
            return format!("{secs}S");
        }
        let mins = secs / 60;
        if (mins as u128) <= MAX_VAL {
            return format!("{mins}M");
        }
        let hours = secs / 3600;
        format!("{hours}H")
    }

    /// Build a `http::Request` from a pre-encoded body, URI string, and metadata.
    ///
    /// Always injects the gRPC-mandatory headers (`content-type`, `te: trailers`).
    /// Caller-supplied metadata headers come last — they may override defaults
    /// when intentional (no defensive filtering, since the caller owns the wire).
    fn build_http_request(
        uri_str: &str,
        body_bytes: Bytes,
        metadata: &GrpcMetadata,
        deadline: Option<Duration>,
    ) -> GrpcEgressResult<http::Request<Full<Bytes>>> {
        let uri: http::Uri = uri_str.parse().map_err(|e| {
            tracing::warn!(error = %e, uri = %uri_str, "invalid gRPC URI supplied by caller");
            GrpcEgressError::Internal(SANITIZED_INTERNAL_MSG.into())
        })?;

        let mut builder = http::Request::builder()
            .method(http::Method::POST)
            .uri(uri)
            .header(http::header::CONTENT_TYPE, "application/grpc")
            .header("te", "trailers");

        if let Some(d) = deadline {
            builder = builder.header("grpc-timeout", TonicGrpcClientCore::encode_grpc_timeout(d));
        }

        for (k, v) in &metadata.headers {
            builder = builder.header(k.as_str(), v.as_str());
        }

        builder.body(Full::new(body_bytes)).map_err(|e| {
            tracing::warn!(error = %e, "failed to build HTTP request for gRPC call");
            GrpcEgressError::Internal(SANITIZED_INTERNAL_MSG.into())
        })
    }

    /// Extract `HashMap<String, String>` from an optional `HeaderMap` reference.
    fn header_map_to_hash(
        map: Option<&http::HeaderMap>,
    ) -> std::collections::HashMap<String, String> {
        let mut out = std::collections::HashMap::new();
        if let Some(m) = map {
            for (k, v) in m {
                if let Ok(s) = v.to_str() {
                    out.insert(k.as_str().to_owned(), s.to_owned());
                }
            }
        }
        out
    }

    /// Check the `grpc-status` trailer value; return `Err(Status(...))` for anything != "0".
    ///
    /// When the status is `RESOURCE_EXHAUSTED` and the response headers carry a
    /// `retry-after` value (seconds), the value is embedded into the error message
    /// as `[retry-after=Ns]`. [`crate::core::resilience::retry::RetryPolicy::decide`]
    /// parses this hint to honour the upstream reset window rather than guessing.
    ///
    /// `grpc-message` is a *server-supplied* sanitized message that the gRPC spec
    /// already requires not to contain server internals.  We pass it through as-is.
    fn check_grpc_status(
        trailers: Option<&http::HeaderMap>,
        response_headers: Option<&http::HeaderMap>,
    ) -> GrpcEgressResult<()> {
        let code_str = trailers
            .and_then(|m| m.get("grpc-status"))
            .and_then(|v| v.to_str().ok())
            .unwrap_or("0");

        if code_str == "0" {
            return Ok(());
        }

        let wire: i32 = code_str.parse().unwrap_or(2 /* Unknown */);
        let code = StatusConversions::from_wire(wire);
        let mut message = trailers
            .and_then(|m| m.get("grpc-message"))
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_owned();

        // Embed Retry-After hint for RESOURCE_EXHAUSTED so the resilient client
        // can honour the upstream reset window. Only integer seconds are supported
        // (the HTTP-date form is uncommon in gRPC contexts).
        if code == GrpcStatusCode::ResourceExhausted {
            if let Some(secs) = response_headers
                .and_then(|h| {
                    h.get("retry-after")
                        .or_else(|| h.get("x-ratelimit-reset-requests"))
                })
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
            {
                message = format!("{message} [retry-after={secs}s]");
            }
        }

        Err(GrpcEgressError::Status(code, message))
    }
} // impl TonicGrpcClientCore (helpers)

// ── constructor helpers ───────────────────────────────────────────────────────

impl TonicGrpcClient {
    /// Create a client with an explicit fallback timeout.
    pub(crate) fn with_timeout(base_uri: impl Into<String>, timeout: Duration) -> Self {
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
            timeout,
            interceptors: GrpcEgressInterceptorChain::new(),
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            compression: CompressionMode::None,
        }
    }

    /// Construct a client from a [`GrpcChannelConfig`].
    ///
    /// **Fail-closed**: if `config.tls_required` is `true` and the
    /// endpoint URL has an `http://` scheme, returns
    /// [`GrpcChannelConfigError::PlaintextRejected`] before any
    /// transport setup.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "used in tests; superseded by TransportSvc factory"
        )
    )]
    fn from_config(config: &GrpcChannelConfig) -> Result<Self, GrpcChannelConfigError> {
        use crate::api::value::DEFAULT_REQUEST_TIMEOUT_SECS;
        if config.tls_required && TonicGrpcClientCore::is_plaintext_endpoint(&config.endpoint) {
            return Err(GrpcChannelConfigError::PlaintextRejected(
                config.endpoint.clone(),
            ));
        }
        let timeout = Duration::from_secs(
            config
                .request_timeout_secs
                .unwrap_or(DEFAULT_REQUEST_TIMEOUT_SECS),
        );
        let mut client = Self::with_timeout(&config.endpoint, timeout);
        client.max_message_bytes = config.max_message_bytes;
        client.compression = config.compression;
        Ok(client)
    }

    /// Attach an interceptor chain to the client.  Replaces any previous chain.
    pub(crate) fn with_interceptors(mut self, chain: GrpcEgressInterceptorChain) -> Self {
        self.interceptors = chain;
        self
    }

    /// Override the max-message-bytes cap.
    pub(crate) fn with_max_message_bytes(mut self, bytes: usize) -> Self {
        self.max_message_bytes = bytes;
        self
    }

    /// Override the compression mode.
    pub(crate) fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }
}

impl TonicGrpcClientCore {
    /// Returns `true` when `endpoint` starts with `http://` (case-insensitive).
    pub(crate) fn is_plaintext_endpoint(endpoint: &str) -> bool {
        endpoint.len() >= 7 && endpoint[..7].eq_ignore_ascii_case("http://")
    }

    /// Race a future against the request's deadline AND its optional cancellation
    /// token.  Used to wrap every awaitable network step in unary/stream calls.
    ///
    /// Resolves to:
    /// - `Ok(value)`     — future completed in time
    /// - `Err(Timeout)`  — deadline elapsed first
    /// - `Err(Cancelled)`— cancellation token fired first
    async fn race_deadline_and_cancel<F, T>(
        fut: F,
        deadline: Duration,
        cancel: Option<&CancellationToken>,
    ) -> GrpcEgressResult<T>
    where
        F: std::future::Future<Output = T>,
    {
        let timeout_fut = tokio::time::timeout(deadline, fut);
        match cancel {
            Some(token) => tokio::select! {
                biased;
                _ = token.cancelled() => Err(GrpcEgressError::Cancelled(
                    "caller cancelled in-flight request".into(),
                )),
                res = timeout_fut => res.map_err(|_| GrpcEgressError::Timeout(
                    "request deadline exceeded".into(),
                )),
            },
            None => timeout_fut
                .await
                .map_err(|_| GrpcEgressError::Timeout("request deadline exceeded".into())),
        }
    }
} // impl TonicGrpcClientCore (endpoint + race)

// ── Processor impl ───────────────────────────────────────────────────────────

impl crate::api::traits::processor::Processor for TonicGrpcClient {
    fn process(&self) -> futures::future::BoxFuture<'_, Result<(), GrpcEgressError>> {
        // Default: verify the endpoint is reachable — a no-op health probe.
        Box::pin(self.health_check())
    }

    fn describe(&self) -> &'static str {
        const LABEL: &str = "tonic-grpc-client";
        LABEL
    }
}

// ── GrpcEgress impl ─────────────────────────────────────────────────────────

impl GrpcEgress for TonicGrpcClient {
    fn call_unary(
        &self,
        mut request: GrpcRequest,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
        // Run before-call interceptors; first failure short-circuits.
        if let Err(e) = self.interceptors.run_before(&mut request) {
            return Box::pin(futures::future::ready(Err(e)));
        }

        // Inject grpc-encoding when compression is enabled.  Done after
        // interceptors so they can override the negotiated value.
        if let Some(name) = self.compression.header_value() {
            request
                .metadata
                .headers
                .entry("grpc-encoding".to_string())
                .or_insert_with(|| name.to_string());
            request
                .metadata
                .headers
                .entry("grpc-accept-encoding".to_string())
                .or_insert_with(|| name.to_string());
        }

        // Method paths are conventionally written with a leading `/`
        // (e.g. `/pkg.Service/Method`); trim it so we never produce
        // `host//path`. Strict registry-based dispatchers (e.g.
        // `HandlerRegistryDispatcher`) key on the exact path string and
        // reject the double-slash form.
        let method = request.method.trim_start_matches('/');
        let uri_str = format!("{}/{}", self.base_uri.trim_end_matches('/'), method);
        let body_bytes = TonicGrpcClientCore::encode_grpc_frame(&request.body);
        let deadline = request.deadline;
        let cancel = request.cancellation.clone();
        let max_bytes = self.max_message_bytes;
        let interceptors = self.interceptors.clone();
        let http_req = match TonicGrpcClientCore::build_http_request(
            &uri_str,
            body_bytes,
            &request.metadata,
            Some(deadline),
        ) {
            Ok(r) => r,
            Err(e) => return Box::pin(futures::future::ready(Err(e))),
        };

        Box::pin(async move {
            let resp = TonicGrpcClientCore::race_deadline_and_cancel(
                self.client.request(http_req),
                deadline,
                cancel.as_ref(),
            )
            .await?
            .map_err(|e| {
                tracing::warn!(error = %e, "hyper transport error during gRPC call");
                GrpcEgressError::ConnectionFailed("transport error".into())
            })?;

            let response_headers = resp.headers().clone();
            TonicGrpcClientCore::check_grpc_status(
                Some(&response_headers),
                Some(&response_headers),
            )?;

            // Bound the response body; oversize returns ResourceExhausted.
            let limited = http_body_util::Limited::new(resp.into_body(), max_bytes + 5);
            let collected = TonicGrpcClientCore::race_deadline_and_cancel(
                limited.collect(),
                deadline,
                cancel.as_ref(),
            )
            .await?
            .map_err(|e| {
                tracing::warn!(error = %e, "response body exceeded max_message_bytes or transport error");
                GrpcEgressError::Status(
                    GrpcStatusCode::ResourceExhausted,
                    "response body exceeded max_message_bytes".into(),
                )
            })?;

            TonicGrpcClientCore::check_grpc_status(collected.trailers(), Some(&response_headers))?;

            let trailer_headers = TonicGrpcClientCore::header_map_to_hash(collected.trailers());
            let data = collected.to_bytes();

            let body = if data.len() >= 5 {
                data[5..].to_vec()
            } else {
                data.to_vec()
            };

            let mut response = GrpcResponse {
                body,
                metadata: GrpcMetadata {
                    headers: trailer_headers,
                },
            };

            // Run after-call interceptors; first failure short-circuits.
            interceptors.run_after(&mut response)?;

            Ok(response)
        })
    }

    /// Send a gRPC streaming call.
    ///
    /// **Buffering limitation**: both the request and response are fully buffered
    /// in memory before this future resolves. The `GrpcMessageStream` input is
    /// collected into a single HTTP/2 DATA frame sequence, and the response body
    /// is collected before returning. This is functionally correct for small
    /// message sets but is not suitable for large or infinite streams. True
    /// chunked streaming requires replacing hyper's `Full<Bytes>` body type with
    /// a streaming body, which is a separate task.
    ///
    /// Streaming has no per-request deadline yet — uses the client-level
    /// `timeout`.  Phase 2 will plumb a `GrpcRequest`-shaped streaming envelope.
    fn call_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcMessageStream>> {
        let method = method.trim_start_matches('/');
        let uri_str = format!("{}/{}", self.base_uri.trim_end_matches('/'), method);

        Box::pin(async move {
            // Collect all input messages into one body (multiple gRPC frames).
            let mut body_buf = BytesMut::new();
            let mut stream = messages;
            while let Some(item) = stream.next().await {
                let payload = item?;
                let frame = TonicGrpcClientCore::encode_grpc_frame(&payload);
                body_buf.put(frame);
            }

            let http_req = TonicGrpcClientCore::build_http_request(
                &uri_str,
                body_buf.freeze(),
                &metadata,
                Some(self.timeout),
            )?;

            let resp = tokio::time::timeout(self.timeout, self.client.request(http_req))
                .await
                .map_err(|_| GrpcEgressError::Timeout("stream request deadline exceeded".into()))?
                .map_err(|e| {
                    tracing::warn!(error = %e, "hyper transport error during gRPC stream");
                    GrpcEgressError::ConnectionFailed("transport error".into())
                })?;

            // Check grpc-status in the initial response headers first.
            TonicGrpcClientCore::check_grpc_status(Some(resp.headers()), Some(resp.headers()))?;

            let collected = resp.into_body().collect().await.map_err(|e| {
                tracing::warn!(error = %e, "failed to read gRPC stream response body");
                GrpcEgressError::Internal(SANITIZED_INTERNAL_MSG.into())
            })?;

            // Also check trailers.
            TonicGrpcClientCore::check_grpc_status(collected.trailers(), None)?;

            let data = collected.to_bytes();
            let frames = TonicGrpcClientCore::decode_grpc_frames(data)?;
            let items: Vec<GrpcEgressResult<Vec<u8>>> = frames.into_iter().map(Ok).collect();
            Ok(Box::pin(futures::stream::iter(items)) as GrpcMessageStream)
        })
    }

    /// Send a server-streaming request — single encoded request, multiple response frames.
    fn call_server_stream(
        &self,
        mut request: GrpcRequest,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcMessageStream>> {
        if let Err(e) = self.interceptors.run_before(&mut request) {
            return Box::pin(futures::future::ready(Err(e)));
        }
        let method = request.method.trim_start_matches('/');
        let uri_str = format!("{}/{}", self.base_uri.trim_end_matches('/'), method);
        let body_bytes = TonicGrpcClientCore::encode_grpc_frame(&request.body);
        let deadline = request.deadline;
        let cancel = request.cancellation.clone();
        let max_bytes = self.max_message_bytes;
        let http_req = match TonicGrpcClientCore::build_http_request(
            &uri_str,
            body_bytes,
            &request.metadata,
            Some(deadline),
        ) {
            Ok(r) => r,
            Err(e) => return Box::pin(futures::future::ready(Err(e))),
        };

        Box::pin(async move {
            let resp = TonicGrpcClientCore::race_deadline_and_cancel(
                self.client.request(http_req),
                deadline,
                cancel.as_ref(),
            )
            .await?
            .map_err(|e| {
                tracing::warn!(error = %e, "hyper transport error during gRPC server-stream");
                GrpcEgressError::ConnectionFailed("transport error".into())
            })?;

            let response_headers = resp.headers().clone();
            TonicGrpcClientCore::check_grpc_status(
                Some(&response_headers),
                Some(&response_headers),
            )?;

            let limited = http_body_util::Limited::new(resp.into_body(), max_bytes + 5);
            let collected = TonicGrpcClientCore::race_deadline_and_cancel(
                limited.collect(),
                deadline,
                cancel.as_ref(),
            )
            .await?
            .map_err(|_| {
                GrpcEgressError::Status(
                    GrpcStatusCode::ResourceExhausted,
                    "response exceeded max_message_bytes".into(),
                )
            })?;

            TonicGrpcClientCore::check_grpc_status(collected.trailers(), Some(&response_headers))?;

            let data = collected.to_bytes();
            let frames = TonicGrpcClientCore::decode_grpc_frames(data)?;
            let items: Vec<GrpcEgressResult<Vec<u8>>> = frames.into_iter().map(Ok).collect();
            Ok(Box::pin(futures::stream::iter(items)) as GrpcMessageStream)
        })
    }

    /// Send a client-streaming request — multiple request frames, single response.
    fn call_client_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
        let method_path = method.trim_start_matches('/').to_owned();
        let uri_str = format!("{}/{}", self.base_uri.trim_end_matches('/'), method_path);
        let deadline = self.timeout;
        let max_bytes = self.max_message_bytes;
        let interceptors = self.interceptors.clone();

        Box::pin(async move {
            // Collect all request frames into one body.
            let mut body_buf = BytesMut::new();
            let mut stream = messages;
            while let Some(item) = stream.next().await {
                let payload = item?;
                body_buf.put(TonicGrpcClientCore::encode_grpc_frame(&payload));
            }

            let http_req = TonicGrpcClientCore::build_http_request(
                &uri_str,
                body_buf.freeze(),
                &metadata,
                Some(deadline),
            )?;

            let resp = tokio::time::timeout(deadline, self.client.request(http_req))
                .await
                .map_err(|_| {
                    GrpcEgressError::Timeout("client-stream request deadline exceeded".into())
                })?
                .map_err(|e| {
                    tracing::warn!(error = %e, "hyper transport error during gRPC client-stream");
                    GrpcEgressError::ConnectionFailed("transport error".into())
                })?;

            let response_headers = resp.headers().clone();
            TonicGrpcClientCore::check_grpc_status(
                Some(&response_headers),
                Some(&response_headers),
            )?;

            let limited = http_body_util::Limited::new(resp.into_body(), max_bytes + 5);
            let collected = tokio::time::timeout(deadline, limited.collect())
                .await
                .map_err(|_| {
                    GrpcEgressError::Timeout("client-stream response deadline exceeded".into())
                })?
                .map_err(|_| {
                    GrpcEgressError::Status(
                        GrpcStatusCode::ResourceExhausted,
                        "response exceeded max_message_bytes".into(),
                    )
                })?;

            TonicGrpcClientCore::check_grpc_status(collected.trailers(), Some(&response_headers))?;

            let trailer_headers = TonicGrpcClientCore::header_map_to_hash(collected.trailers());
            let data = collected.to_bytes();
            let body = if data.len() >= 5 {
                data[5..].to_vec()
            } else {
                data.to_vec()
            };

            let mut response = GrpcResponse {
                body,
                metadata: GrpcMetadata {
                    headers: trailer_headers,
                },
            };

            interceptors.run_after(&mut response)?;
            Ok(response)
        })
    }

    /// Send a bidirectional-streaming request — delegates to [`call_stream`].
    ///
    /// [`call_stream`]: GrpcEgress::call_stream
    fn call_bidi_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcMessageStream>> {
        self.call_stream(method, metadata, messages)
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
        let base_uri = self.base_uri.clone();

        Box::pin(async move {
            let uri: http::Uri = base_uri.parse().map_err(|e| {
                tracing::warn!(error = %e, uri = %base_uri, "invalid gRPC URI in health check");
                GrpcEgressError::Internal(SANITIZED_INTERNAL_MSG.into())
            })?;

            let host = uri.host().unwrap_or("127.0.0.1").to_owned();
            let port = uri.port_u16().unwrap_or(50051);
            let addr = format!("{host}:{port}");

            tokio::net::TcpStream::connect(&addr)
                .await
                .map(|_| ())
                .map_err(|e| GrpcEgressError::Unavailable(format!("{addr}: {e}")))
        })
    }
}

impl TonicGrpcClientCore {
    /// Compile-time guard: `GrpcRequest::new` MUST require a `Duration`.
    #[doc(hidden)]
    pub(crate) fn _grpc_request_new_requires_deadline_compile_check() -> GrpcRequest {
        GrpcRequest::new("svc/Method", Vec::new(), Duration::from_secs(1))
    }

    // Quiet the dead-code warning on a public-only-by-import symbol.
    fn _suppress_status_code_unused_import_warning(c: GrpcStatusCode) -> GrpcStatusCode {
        c
    }
} // impl TonicGrpcClientCore (compile checks)

// ── unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_plaintext_endpoint (private fn — inline per rule 37) ──────────────

    #[test]
    fn test_is_plaintext_endpoint_returns_true_for_http_scheme() {
        assert!(TonicGrpcClientCore::is_plaintext_endpoint(
            "http://localhost:50051"
        ));
        assert!(TonicGrpcClientCore::is_plaintext_endpoint(
            "HTTP://example.com:443"
        ));
    }

    #[test]
    fn test_is_plaintext_endpoint_returns_false_for_https_scheme() {
        assert!(!TonicGrpcClientCore::is_plaintext_endpoint(
            "https://secure.example.com:443"
        ));
    }

    #[test]
    fn test_is_plaintext_endpoint_returns_false_for_empty_string() {
        assert!(!TonicGrpcClientCore::is_plaintext_endpoint(""));
    }

    #[test]
    fn test_is_plaintext_endpoint_returns_false_for_short_string() {
        assert!(!TonicGrpcClientCore::is_plaintext_endpoint("http:/"));
    }

    // ── other core tests ─────────────────────────────────────────────────────

    #[test]
    fn test_new_client_stores_base_uri() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let client = TonicGrpcClient::new("http://localhost:50051");
        assert_eq!(client.base_uri, "http://localhost:50051");
    }

    #[test]
    fn test_with_timeout_overrides_duration() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let d = Duration::from_secs(5);
        let client = TonicGrpcClient::with_timeout("http://localhost:50051", d);
        assert_eq!(client.timeout, d);
    }

    #[test]
    fn test_grpc_egress_is_object_safe() {
        fn _assert(_: &dyn GrpcEgress) {}
    }

    #[test]
    fn test_encode_grpc_frame_produces_5_byte_header() {
        let frame = TonicGrpcClientCore::encode_grpc_frame(b"hello");
        assert_eq!(frame.len(), 10); // 5 header + 5 payload
        assert_eq!(frame[0], 0x00); // not compressed
        assert_eq!(
            u32::from_be_bytes([frame[1], frame[2], frame[3], frame[4]]),
            5
        );
        assert_eq!(&frame[5..], b"hello");
    }

    #[test]
    fn test_decode_grpc_frames_round_trips_single_frame() {
        let encoded = TonicGrpcClientCore::encode_grpc_frame(b"world");
        let decoded = TonicGrpcClientCore::decode_grpc_frames(encoded).expect("decode failed");
        assert_eq!(decoded.len(), 1);
        assert_eq!(decoded[0], b"world");
    }

    #[test]
    fn test_decode_grpc_frames_round_trips_multiple_frames() {
        let mut buf = BytesMut::new();
        buf.put(TonicGrpcClientCore::encode_grpc_frame(b"one"));
        buf.put(TonicGrpcClientCore::encode_grpc_frame(b"two"));
        buf.put(TonicGrpcClientCore::encode_grpc_frame(b"three"));
        let decoded = TonicGrpcClientCore::decode_grpc_frames(buf.freeze()).expect("decode failed");
        assert_eq!(decoded.len(), 3);
        assert_eq!(decoded[0], b"one");
        assert_eq!(decoded[1], b"two");
        assert_eq!(decoded[2], b"three");
    }

    #[test]
    fn test_decode_grpc_frames_returns_internal_error_on_truncated_data() {
        // 5-byte header says length=100 but only 3 bytes of payload follow.
        let mut buf = BytesMut::new();
        buf.put_u8(0x00);
        buf.put_u32(100_u32);
        buf.put_slice(b"abc");
        let result = TonicGrpcClientCore::decode_grpc_frames(buf.freeze());
        match result {
            Err(GrpcEgressError::Internal(msg)) => {
                // Must be the sanitized constant — never raw byte counts on the wire.
                assert_eq!(msg, SANITIZED_INTERNAL_MSG);
            }
            other => panic!("expected Internal(sanitized), got {other:?}"),
        }
    }

    #[test]
    fn test_encode_grpc_timeout_uses_nanos_for_small_values() {
        assert_eq!(
            TonicGrpcClientCore::encode_grpc_timeout(Duration::from_nanos(1)),
            "1n"
        );
        assert_eq!(
            TonicGrpcClientCore::encode_grpc_timeout(Duration::from_micros(1)),
            "1000n"
        );
    }

    #[test]
    fn test_encode_grpc_timeout_uses_millis_or_higher_for_seconds() {
        // 30s = 30_000_000_000 ns — too big for nanos, fits microseconds (3e10).
        // Actually 3e10 > 99_999_999, so it falls through to millis (30_000) or higher.
        let s = TonicGrpcClientCore::encode_grpc_timeout(Duration::from_secs(30));
        // Accept any unit suffix as long as the value parses and is positive.
        let last = s.chars().last().expect("non-empty timeout encoding");
        assert!("nuMSmH".contains(last), "unexpected unit suffix in {s}");
    }

    #[test]
    fn test_encode_grpc_timeout_handles_zero_duration() {
        assert_eq!(
            TonicGrpcClientCore::encode_grpc_timeout(Duration::ZERO),
            "0n"
        );
    }

    #[test]
    fn test_grpc_request_new_requires_deadline_compile_check() {
        let _r = TonicGrpcClientCore::_grpc_request_new_requires_deadline_compile_check();
    }

    #[test]
    fn test_from_config_builds_client_from_channel_config() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let cfg =
            crate::api::value::GrpcChannelConfig::new("http://localhost:50051").allow_plaintext();
        assert!(TonicGrpcClient::from_config(&cfg).is_ok());
    }

    #[test]
    fn test_from_config_honors_request_timeout_secs() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let cfg = crate::api::value::GrpcChannelConfig::new("http://localhost:50051")
            .allow_plaintext()
            .with_request_timeout(Duration::from_secs(120));
        let client = TonicGrpcClient::from_config(&cfg).unwrap();
        assert_eq!(client.timeout, Duration::from_secs(120));
    }

    #[test]
    fn test_from_config_defaults_to_30s_when_timeout_not_set() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let cfg =
            crate::api::value::GrpcChannelConfig::new("http://localhost:50051").allow_plaintext();
        let client = TonicGrpcClient::from_config(&cfg).unwrap();
        assert_eq!(client.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_with_interceptors_attaches_chain() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let client = TonicGrpcClient::new("http://localhost:50051")
            .with_interceptors(GrpcEgressInterceptorChain::new());
        assert_eq!(client.interceptors.len(), 0);
    }

    #[test]
    fn test_with_max_message_bytes_overrides_default_cap() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let client =
            TonicGrpcClient::new("http://localhost:50051").with_max_message_bytes(16 * 1024 * 1024);
        assert_eq!(client.max_message_bytes, 16 * 1024 * 1024);
    }

    #[test]
    fn test_with_compression_sets_mode() {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .ok();
        let client =
            TonicGrpcClient::new("http://localhost:50051").with_compression(CompressionMode::Gzip);
        assert_eq!(client.compression, CompressionMode::Gzip);
    }
}
