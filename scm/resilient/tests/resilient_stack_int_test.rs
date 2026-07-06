#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for the resilient gRPC transport stack.
//!
//! These tests use an in-memory [`CountingMock`] that implements
//! [`GrpcEgress`] so the retry and circuit-breaker logic runs against
//! configurable response sequences without a real network connection.

use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::Duration;

use futures::future::BoxFuture;
use swe_edge_egress_grpc::{
    GrpcChannelConfig, GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcMetadata, GrpcRequest,
    GrpcResponse, GrpcStatusCode, HealthCheckRequest, ResilienceConfig,
};
use swe_edge_egress_grpc_breaker::{BreakerState, GrpcBreakerClient, GrpcBreakerConfig};
use swe_edge_egress_grpc_resilient::{GrpcResilientFacade, ResilientTransportError};
use swe_edge_egress_grpc_retry::{GrpcRetryClient, GrpcRetryConfig};

// ── helpers ──────────────────────────────────────────────────────────────────

fn ensure_tls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

fn ok_response() -> GrpcEgressResult<GrpcResponse> {
    Ok(GrpcResponse {
        body: vec![],
        metadata: GrpcMetadata::default(),
    })
}

fn unavailable() -> GrpcEgressResult<GrpcResponse> {
    Err(GrpcEgressError::Status(
        GrpcStatusCode::Unavailable,
        "upstream down".into(),
    ))
}

fn hard_quota() -> GrpcEgressResult<GrpcResponse> {
    Err(GrpcEgressError::Status(
        GrpcStatusCode::ResourceExhausted,
        "quota exceeded".into(),
    ))
}

fn rate_limited() -> GrpcEgressResult<GrpcResponse> {
    Err(GrpcEgressError::Status(
        GrpcStatusCode::ResourceExhausted,
        "rate limit exceeded".into(),
    ))
}

fn req() -> GrpcRequest {
    GrpcRequest::new("svc/Method", vec![], Duration::from_secs(10))
}

fn valid_resilience() -> ResilienceConfig {
    ResilienceConfig {
        max_attempts: 3,
        initial_backoff_ms: 10,
        backoff_multiplier: 2.0,
        jitter_factor: 0.0,
        max_backoff_ms: 100,
        rate_limit_max_attempts: 2,
        rate_limit_initial_backoff_ms: 10,
        rate_limit_max_backoff_ms: 100,
        failure_threshold: 5,
        cool_down_seconds: 10,
        half_open_probe_count: 1,
    }
}

// ── CountingMock ─────────────────────────────────────────────────────────────

// @allow: no_mocks_in_integration — CountingMock is an in-process test double
// for the GrpcEgress transport layer, not an external service mock. It
// exercises the real retry and circuit-breaker logic with deterministic
// responses, which is only possible via a programmatic stand-in.
/// In-memory mock: counts calls and returns pre-loaded responses in order.
struct CountingMock {
    hits: Arc<AtomicUsize>,
    queue: Arc<Mutex<VecDeque<GrpcEgressResult<GrpcResponse>>>>,
}

impl CountingMock {
    fn with_responses(
        hits: Arc<AtomicUsize>,
        responses: Vec<GrpcEgressResult<GrpcResponse>>,
    ) -> Self {
        Self {
            hits,
            queue: Arc::new(Mutex::new(responses.into_iter().collect())),
        }
    }
}

impl GrpcEgress for CountingMock {
    fn call_unary(&self, _req: GrpcRequest) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
        self.hits.fetch_add(1, Ordering::SeqCst);
        let result = self
            .queue
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(ok_response);
        Box::pin(futures::future::ready(result))
    }

    fn health_check(&self, _req: HealthCheckRequest) -> BoxFuture<'_, GrpcEgressResult<()>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

// Minimal retry config suitable for fast tests (1 ms backoff).
fn fast_retry(max_attempts: u32) -> GrpcRetryConfig {
    GrpcRetryConfig {
        max_attempts,
        initial_backoff_ms: 1,
        backoff_multiplier: 1.0,
        jitter_factor: 0.0,
        max_backoff_ms: 1,
        rate_limit_max_attempts: 2,
        rate_limit_initial_backoff_ms: 1,
        rate_limit_max_backoff_ms: 1,
    }
}

// ── factory smoke tests ───────────────────────────────────────────────────────

/// @covers: create_resilient_transport_from_config
#[tokio::test]
async fn test_create_resilient_transport_from_config_without_resilience_returns_ok_happy() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
    let transport = GrpcResilientFacade::create_resilient_transport_from_config(&config)
        .expect("assembly must succeed for a valid plaintext config");
    // Nothing listens on 127.0.0.1:50051 in the test environment, so a real
    // call must genuinely fail — proves this is a connectable client, not a stub.
    let health = transport.health_check(HealthCheckRequest).await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}

/// @covers: create_resilient_transport_from_config
#[tokio::test]
async fn test_create_resilient_transport_from_config_with_valid_resilience_returns_ok_happy() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(valid_resilience());
    let transport = GrpcResilientFacade::create_resilient_transport_from_config(&config)
        .expect("assembly must succeed for a valid resilience config");
    let health = transport.health_check(HealthCheckRequest).await;
    assert!(
        matches!(health, Err(GrpcEgressError::Unavailable(_))),
        "health_check against an unbound port must report Unavailable, got: {health:?}"
    );
}

/// @covers: create_resilient_transport_from_config
#[test]
fn test_create_resilient_transport_from_config_tls_required_rejects_plaintext_endpoint_error() {
    ensure_tls_provider();
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
    assert!(matches!(
        GrpcResilientFacade::create_resilient_transport_from_config(&config),
        Err(ResilientTransportError::ChannelConfig(_))
    ));
}

/// @covers: create_resilient_transport_from_config
#[test]
fn test_create_resilient_transport_from_config_invalid_resilience_returns_error_edge() {
    ensure_tls_provider();
    let mut r = valid_resilience();
    r.max_attempts = 0;
    let config = GrpcChannelConfig::new("http://127.0.0.1:50051")
        .allow_plaintext()
        .with_resilience(r);
    assert!(matches!(
        GrpcResilientFacade::create_resilient_transport_from_config(&config),
        Err(ResilientTransportError::InvalidResilience(_))
    ));
}

// ── retry amplification ───────────────────────────────────────────────────────

/// @covers: retry amplification — factor ≤ 1.5× under 20% failure rate
///
/// 10 client calls; 2 fail once then succeed on the first retry.
/// Expected server hits: 10 + 2 = 12 (1.2× amplification ≤ 1.5× threshold).
#[tokio::test]
async fn test_retry_amplification_factor_does_not_exceed_one_point_five_times() {
    let hits = Arc::new(AtomicUsize::new(0));
    // Responses in pop order:
    // call 1: ok              → 1 hit
    // call 2: unavail → ok    → 2 hits (1 retry)
    // calls 3-8: ok ×6        → 6 hits
    // call 9: unavail → ok    → 2 hits (1 retry)
    // call 10: ok             → 1 hit
    // total: 12 hits for 10 calls → 1.2×
    let responses = vec![
        ok_response(),
        unavailable(),
        ok_response(),
        ok_response(),
        ok_response(),
        ok_response(),
        ok_response(),
        ok_response(),
        ok_response(),
        unavailable(),
        ok_response(),
        ok_response(),
    ];
    let mock = CountingMock::with_responses(Arc::clone(&hits), responses);
    let breaker_cfg = GrpcBreakerConfig {
        failure_threshold: 100,
        cool_down_seconds: 60,
        half_open_probe_count: 1,
    };
    let client = GrpcBreakerClient::new(GrpcRetryClient::new(mock, fast_retry(3)), breaker_cfg);

    for _ in 0..10_usize {
        let _ = client.call_unary(req()).await;
    }

    let server_hits = hits.load(Ordering::SeqCst);
    let n_calls = 10_usize;
    let amplification = server_hits as f64 / n_calls as f64;
    assert!(
        amplification <= 1.5,
        "amplification {amplification:.2}× exceeds 1.5× ({server_hits} hits / {n_calls} calls)",
    );
}

// ── retry exhaustion ─────────────────────────────────────────────────────────

/// @covers: retry — all attempts exhausted returns the last error
///
/// max_attempts=3, all 3 fail with Unavailable.
/// Server is hit exactly 3 times; result is Err(Unavailable or Status(Unavailable)).
#[tokio::test]
async fn test_unavailable_error_is_retried_up_to_max_attempts_then_fails() {
    let hits = Arc::new(AtomicUsize::new(0));
    let responses = vec![unavailable(), unavailable(), unavailable()];
    let mock = CountingMock::with_responses(Arc::clone(&hits), responses);
    let breaker_cfg = GrpcBreakerConfig {
        failure_threshold: 100,
        cool_down_seconds: 60,
        half_open_probe_count: 1,
    };
    let client = GrpcBreakerClient::new(GrpcRetryClient::new(mock, fast_retry(3)), breaker_cfg);

    let result = client.call_unary(req()).await;
    assert!(
        result.is_err(),
        "expected error after exhausting all attempts"
    );
    assert_eq!(
        hits.load(Ordering::SeqCst),
        3,
        "expected exactly 3 server hits"
    );
}

// ── hard quota — never retried ────────────────────────────────────────────────

/// @covers: retry — HardQuota ResourceExhausted is never retried
///
/// max_attempts=3 would allow up to 3 hits, but HardQuota is Terminal.
/// Server must be hit exactly once.
#[tokio::test]
async fn test_hard_quota_resource_exhausted_is_not_retried() {
    let hits = Arc::new(AtomicUsize::new(0));
    let responses = vec![hard_quota(), ok_response(), ok_response()]; // extra ok's unused
    let mock = CountingMock::with_responses(Arc::clone(&hits), responses);
    let breaker_cfg = GrpcBreakerConfig {
        failure_threshold: 100,
        cool_down_seconds: 60,
        half_open_probe_count: 1,
    };
    let client = GrpcBreakerClient::new(GrpcRetryClient::new(mock, fast_retry(3)), breaker_cfg);

    let result = client.call_unary(req()).await;
    assert!(result.is_err(), "expected quota error");
    assert_eq!(
        hits.load(Ordering::SeqCst),
        1,
        "HardQuota must not be retried — server should see exactly 1 hit",
    );
}

// ── rate-limit retry track ────────────────────────────────────────────────────

/// @covers: retry — rate-limit error uses rate_limit_max_attempts track
///
/// rate_limit_max_attempts=2, standard max_attempts=5.
/// Semantics: 1 initial call + rate_limit_max_attempts retries = 3 total server hits,
/// which is less than the standard max_attempts ceiling of 5.
#[tokio::test]
async fn test_rate_limit_error_uses_rate_limit_retry_track_not_standard() {
    let hits = Arc::new(AtomicUsize::new(0));
    // Queue 5 rate-limit responses; only 3 should be consumed (1 + 2 retries).
    let responses = vec![
        rate_limited(),
        rate_limited(),
        rate_limited(),
        rate_limited(),
        rate_limited(),
    ];
    let mock = CountingMock::with_responses(Arc::clone(&hits), responses);
    let retry_cfg = GrpcRetryConfig {
        max_attempts: 5,
        initial_backoff_ms: 1,
        backoff_multiplier: 1.0,
        jitter_factor: 0.0,
        max_backoff_ms: 1,
        rate_limit_max_attempts: 2,
        rate_limit_initial_backoff_ms: 1,
        rate_limit_max_backoff_ms: 1,
    };
    let breaker_cfg = GrpcBreakerConfig {
        failure_threshold: 100,
        cool_down_seconds: 60,
        half_open_probe_count: 1,
    };
    let client = GrpcBreakerClient::new(GrpcRetryClient::new(mock, retry_cfg), breaker_cfg);

    let result = client.call_unary(req()).await;
    assert!(result.is_err());
    // rate_limit_max_attempts=2: 1 initial + 2 retries = 3 total server hits (not 5).
    assert_eq!(
        hits.load(Ordering::SeqCst),
        3,
        "rate-limit track: 1 initial + rate_limit_max_attempts(2) retries = 3 total hits",
    );
}

// ── circuit breaker — opens at threshold ─────────────────────────────────────

/// @covers: circuit breaker — opens after failure_threshold consecutive failures
///
/// failure_threshold=3, max_attempts=1.
/// After 3 failing calls, breaker is Open.
/// 4th call fast-fails without touching the server.
#[tokio::test]
async fn test_circuit_breaker_opens_after_consecutive_failures_at_threshold() {
    let hits = Arc::new(AtomicUsize::new(0));
    let responses = vec![unavailable(), unavailable(), unavailable(), unavailable()];
    let mock = CountingMock::with_responses(Arc::clone(&hits), responses);
    let breaker_cfg = GrpcBreakerConfig {
        failure_threshold: 3,
        cool_down_seconds: 60,
        half_open_probe_count: 1,
    };
    let client = GrpcBreakerClient::new(GrpcRetryClient::new(mock, fast_retry(1)), breaker_cfg);

    for _ in 0..3 {
        let _ = client.call_unary(req()).await;
    }

    assert_eq!(
        hits.load(Ordering::SeqCst),
        3,
        "expected 3 server hits to trip the breaker"
    );
    assert!(
        matches!(client.state().await, BreakerState::Open { .. }),
        "breaker must be Open after threshold failures",
    );

    // 4th call: breaker rejects without server hit.
    let result = client.call_unary(req()).await;
    assert!(
        matches!(result, Err(GrpcEgressError::Unavailable(_))),
        "breaker must return Unavailable when Open, got {result:?}",
    );
    assert_eq!(
        hits.load(Ordering::SeqCst),
        3,
        "server hit count must not increase after breaker opens",
    );
}

// ── circuit breaker — recovers via HalfOpen ──────────────────────────────────

/// @covers: circuit breaker — Closed→Open→HalfOpen→Closed transition
///
/// Trips the breaker (3 failures), waits for cool-down (1.1 s), then sends
/// a probe that succeeds — confirming the breaker closes again.
///
/// NOTE: This test takes ≈1.1 s due to the real-time cool-down wait.
#[tokio::test]
async fn test_circuit_breaker_closes_after_probe_succeeds_during_half_open() {
    let hits = Arc::new(AtomicUsize::new(0));
    let responses = vec![
        unavailable(),
        unavailable(),
        unavailable(),
        ok_response(), // probe after cool-down
        ok_response(), // final call to confirm breaker is closed
    ];
    let mock = CountingMock::with_responses(Arc::clone(&hits), responses);
    let breaker_cfg = GrpcBreakerConfig {
        failure_threshold: 3,
        cool_down_seconds: 1,
        half_open_probe_count: 1,
    };
    let client = GrpcBreakerClient::new(GrpcRetryClient::new(mock, fast_retry(1)), breaker_cfg);

    // Trip the breaker.
    for _ in 0..3 {
        let _ = client.call_unary(req()).await;
    }
    assert!(matches!(client.state().await, BreakerState::Open { .. }));

    // Wait for cool-down to expire.
    tokio::time::sleep(Duration::from_millis(1100)).await;

    // Probe: should be admitted (HalfOpen), succeeds, closes breaker.
    let probe = client.call_unary(req()).await;
    assert!(probe.is_ok(), "probe should succeed");
    assert_eq!(
        client.state().await,
        BreakerState::Closed,
        "breaker must be Closed after probe"
    );

    // One more call confirming normal operation.
    let final_result = client.call_unary(req()).await;
    assert!(final_result.is_ok());
    assert_eq!(hits.load(Ordering::SeqCst), 5);
}
