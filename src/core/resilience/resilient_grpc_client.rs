//! `ResilientGrpcClient` — retry + circuit breaker decorator for `GrpcOutbound`.

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::BoxFuture;

use crate::api::port::{GrpcMessageStream, GrpcOutbound, GrpcOutboundError, GrpcOutboundResult};
use crate::api::resilience::circuit_breaker::CircuitBreaker as _;
use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse};
use super::circuit_breaker::CircuitBreaker;
use super::retry::RetryDecision;
use super::retry::RetryPolicy;

/// Retry + circuit breaker decorator over any [`GrpcOutbound`] transport.
///
/// Construct via [`crate::saf::create_transport_from_config`].
///
/// On each `call_unary`:
/// 1. If the circuit is open, fail fast with `Unavailable`.
/// 2. Call the inner transport. On success, record success and return.
/// 3. On error, ask [`RetryPolicy::decide`] — retry with the given backoff
///    (deadline-aware), or propagate immediately.
/// 4. After all attempts exhausted, record a circuit-breaker failure.
///
/// Streaming calls and health checks are passed through without retrying.
pub(crate) struct ResilientGrpcClient {
    inner:   Arc<dyn GrpcOutbound>,
    retry:   RetryPolicy,
    breaker: CircuitBreaker,
}

impl ResilientGrpcClient {
    /// Construct a resilient client wrapping `inner`.
    pub(crate) fn new(
        inner:   Arc<dyn GrpcOutbound>,
        retry:   RetryPolicy,
        breaker: CircuitBreaker,
    ) -> Self {
        Self { inner, retry, breaker }
    }
}

impl std::fmt::Debug for ResilientGrpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResilientGrpcClient")
            .field("retry",   &self.retry)
            .field("breaker", &self.breaker)
            .finish_non_exhaustive()
    }
}

impl crate::api::traits::Processor for ResilientGrpcClient {
    fn describe(&self) -> &'static str { "resilient-grpc-client" }
}

impl GrpcOutbound for ResilientGrpcClient {
    fn call_unary(
        &self,
        request: GrpcRequest,
    ) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
        Box::pin(async move {
            if self.breaker.is_open() {
                tracing::warn!(
                    method = %request.method,
                    "circuit open — failing fast without downstream call"
                );
                return Err(GrpcOutboundError::Unavailable(
                    "circuit open: too many consecutive failures".into(),
                ));
            }

            let original_deadline = request.deadline;
            let start             = Instant::now();
            let mut last_err: Option<GrpcOutboundError> = None;
            let mut retry_index   = 0u32;

            loop {
                let elapsed   = start.elapsed();
                let remaining = match original_deadline.checked_sub(elapsed) {
                    Some(r) if r > Duration::ZERO => r,
                    _ => break, // deadline consumed
                };

                let mut req  = request.clone();
                req.deadline = remaining;

                match self.inner.call_unary(req).await {
                    Ok(resp) => {
                        self.breaker.record_success();
                        return Ok(resp);
                    }
                    Err(e) => {
                        match self.retry.decide(&e, retry_index) {
                            RetryDecision::DoNotRetry => {
                                self.breaker.record_failure();
                                return Err(e);
                            }
                            RetryDecision::Retry(backoff) => {
                                let budget_after_sleep = start.elapsed() + backoff;
                                if budget_after_sleep >= original_deadline {
                                    last_err = Some(e);
                                    break;
                                }
                                tracing::warn!(
                                    method      = %request.method,
                                    retry_index,
                                    backoff_ms  = backoff.as_millis(),
                                    error       = %e,
                                    "retryable gRPC error — backing off"
                                );
                                tokio::time::sleep(backoff).await;
                                last_err     = Some(e);
                                retry_index += 1;
                            }
                        }
                    }
                }
            }

            self.breaker.record_failure();
            Err(last_err.unwrap_or_else(|| {
                GrpcOutboundError::Timeout("deadline exhausted across retry attempts".into())
            }))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
        self.inner.health_check()
    }

    fn call_stream(
        &self,
        method:   String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcOutboundResult<GrpcMessageStream>> {
        // Streaming calls are not retried — partial state makes retry unsafe.
        self.inner.call_stream(method, metadata, messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::{GrpcMetadata, GrpcStatusCode};
    use futures::future::BoxFuture;
    use std::sync::atomic::{AtomicU32, Ordering};

    struct ResilientGrpcClientFakeSuccess {
        call_count: AtomicU32,
        fail_count: u32,
        fail_code:  GrpcStatusCode,
        fail_msg:   &'static str,
    }

    impl ResilientGrpcClientFakeSuccess {
        fn new(fail_count: u32, fail_code: GrpcStatusCode) -> Arc<Self> {
            Arc::new(Self { call_count: AtomicU32::new(0), fail_count, fail_code, fail_msg: "oom" })
        }
        fn with_msg(fail_count: u32, fail_code: GrpcStatusCode, msg: &'static str) -> Arc<Self> {
            Arc::new(Self { call_count: AtomicU32::new(0), fail_count, fail_code, fail_msg: msg })
        }
    }

    impl GrpcOutbound for ResilientGrpcClientFakeSuccess {
        fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
            let n = self.call_count.fetch_add(1, Ordering::SeqCst);
            if n < self.fail_count {
                let code = self.fail_code;
                let msg  = self.fail_msg.to_string();
                Box::pin(futures::future::ready(Err(GrpcOutboundError::Status(code, msg))))
            } else {
                Box::pin(futures::future::ready(Ok(GrpcResponse {
                    body: vec![], metadata: GrpcMetadata::default(),
                })))
            }
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    struct ResilientGrpcClientFakeAlwaysFail { code: GrpcStatusCode, msg: &'static str }
    impl GrpcOutbound for ResilientGrpcClientFakeAlwaysFail {
        fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
            let (code, msg) = (self.code, self.msg.to_string());
            Box::pin(futures::future::ready(Err(GrpcOutboundError::Status(code, msg))))
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    fn req() -> GrpcRequest {
        GrpcRequest::new("svc/Method", vec![], Duration::from_secs(5))
    }

    fn zero_backoff_policy(max_attempts: u32, rate_limit_max_attempts: u32) -> RetryPolicy {
        RetryPolicy {
            max_attempts,
            initial_backoff:            Duration::ZERO,
            backoff_multiplier:         1.0,
            jitter_factor:              0.0,
            max_backoff:                Duration::ZERO,
            rate_limit_max_attempts,
            rate_limit_initial_backoff: Duration::ZERO,
            rate_limit_max_backoff:     Duration::ZERO,
        }
    }

    fn client(inner: Arc<dyn GrpcOutbound>, max_attempts: u32, cb_threshold: u32) -> ResilientGrpcClient {
        ResilientGrpcClient::new(
            inner,
            zero_backoff_policy(max_attempts, 1),
            CircuitBreaker::new(cb_threshold, Duration::from_secs(60), 1),
        )
    }

    #[tokio::test]
    async fn test_succeeds_on_first_attempt() {
        let inner = ResilientGrpcClientFakeSuccess::new(0, GrpcStatusCode::Ok);
        assert!(client(inner.clone(), 3, 10).call_unary(req()).await.is_ok());
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retries_capacity_exhausted_and_succeeds() {
        let inner = ResilientGrpcClientFakeSuccess::new(2, GrpcStatusCode::ResourceExhausted);
        assert!(client(inner.clone(), 3, 10).call_unary(req()).await.is_ok());
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_hard_quota_does_not_retry() {
        let inner = ResilientGrpcClientFakeSuccess::with_msg(5, GrpcStatusCode::ResourceExhausted, "quota exceeded");
        let c = client(inner.clone(), 5, 99);
        assert!(c.call_unary(req()).await.is_err());
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 1, "hard quota must not retry");
    }

    #[tokio::test]
    async fn test_rate_limit_uses_rate_limit_track() {
        // rate_limit_max_attempts=1 → retries once then stops
        let inner = ResilientGrpcClientFakeSuccess::with_msg(5, GrpcStatusCode::ResourceExhausted, "rate limit exceeded");
        let c = client(inner.clone(), 5, 99);
        assert!(c.call_unary(req()).await.is_err());
        // 1 original + 1 rate-limit retry = 2 calls
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 2, "rate-limit track: 1 original + 1 retry");
    }

    #[tokio::test]
    async fn test_internal_does_not_retry() {
        let inner = ResilientGrpcClientFakeSuccess::new(1, GrpcStatusCode::Internal);
        let c = client(inner.clone(), 3, 10);
        assert!(c.call_unary(req()).await.is_err());
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 1, "INTERNAL must not retry");
    }

    #[tokio::test]
    async fn test_circuit_opens_and_fails_fast() {
        let inner: Arc<dyn GrpcOutbound> = Arc::new(ResilientGrpcClientFakeAlwaysFail {
            code: GrpcStatusCode::Internal, msg: "bug",
        });
        let c = client(inner, 1, 2);
        let _ = c.call_unary(req()).await;
        let _ = c.call_unary(req()).await;
        assert!(c.breaker.is_open());
        let err = c.call_unary(req()).await.unwrap_err();
        assert!(
            matches!(err, GrpcOutboundError::Unavailable(ref m) if m.contains("circuit open")),
            "expected circuit-open Unavailable, got {err:?}"
        );
    }

    #[tokio::test]
    async fn test_health_check_delegates_to_inner() {
        let inner = ResilientGrpcClientFakeSuccess::new(0, GrpcStatusCode::Ok);
        assert!(client(inner, 1, 5).health_check().await.is_ok());
    }

    #[test]
    fn test_new_constructs_without_panic() {
        let inner: Arc<dyn GrpcOutbound> = ResilientGrpcClientFakeSuccess::new(0, GrpcStatusCode::Ok);
        let _ = ResilientGrpcClient::new(
            inner,
            zero_backoff_policy(3, 1),
            CircuitBreaker::new(5, Duration::from_secs(60), 1),
        );
    }
}
