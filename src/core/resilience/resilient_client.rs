//! `ResilientGrpcClient` — retry + circuit breaker decorator for `GrpcOutbound`.
//!
//! Wraps any `Arc<dyn GrpcOutbound>` and transparently applies:
//!
//! 1. **Circuit breaker** — if the circuit is open, fail fast with `Unavailable`
//!    rather than attempting the downstream call.
//! 2. **Retry with budget-aware exponential backoff** — on retryable errors
//!    (see [`RetryPolicy::is_retryable`]), sleep and retry, reducing the per-
//!    attempt deadline by elapsed time so the total wall-clock cost never
//!    exceeds the caller's original deadline.
//! 3. **Circuit breaker feedback** — a call that exhausts all retry attempts
//!    records a failure; a successful call resets the consecutive-failure count.

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::BoxFuture;

use crate::api::port::{GrpcMessageStream, GrpcOutbound, GrpcOutboundError, GrpcOutboundResult};
use crate::api::value_object::{GrpcRequest, GrpcResponse};
use super::circuit_breaker::CircuitBreaker;
use super::retry::RetryPolicy;

/// Retry + circuit breaker decorator over any [`GrpcOutbound`] transport.
///
/// Construct via [`crate::saf::create_resilient_transport`]; do not build
/// directly in application code — the SAF factory is the stable API.
pub struct ResilientGrpcClient {
    pub(crate) inner:   Arc<dyn GrpcOutbound>,
    pub(crate) retry:   RetryPolicy,
    pub(crate) breaker: CircuitBreaker,
}

impl std::fmt::Debug for ResilientGrpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResilientGrpcClient")
            .field("retry",   &self.retry)
            .field("breaker", &self.breaker)
            .finish_non_exhaustive()
    }
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

            for retry_index in 0..self.retry.max_attempts {
                let elapsed   = start.elapsed();
                let remaining = match original_deadline.checked_sub(elapsed) {
                    Some(r) if r > Duration::ZERO => r,
                    _ => {
                        // Deadline already consumed — stop immediately.
                        break;
                    }
                };

                let mut req     = request.clone();
                req.deadline    = remaining;
                let is_last     = retry_index + 1 == self.retry.max_attempts;

                match self.inner.call_unary(req).await {
                    Ok(resp) => {
                        self.breaker.record_success();
                        return Ok(resp);
                    }
                    Err(e) if RetryPolicy::is_retryable(&e) && !is_last => {
                        let backoff = self.retry.backoff_for(retry_index);
                        let after_backoff = start.elapsed() + backoff;
                        if after_backoff >= original_deadline {
                            // No budget left after sleeping — give up now.
                            last_err = Some(e);
                            break;
                        }
                        tracing::warn!(
                            method    = %request.method,
                            attempt   = retry_index + 1,
                            max       = self.retry.max_attempts,
                            backoff_ms = backoff.as_millis(),
                            error     = %e,
                            "retryable gRPC error — backing off"
                        );
                        tokio::time::sleep(backoff).await;
                        last_err = Some(e);
                    }
                    Err(e) => {
                        // Non-retryable, or last attempt — record and surface.
                        self.breaker.record_failure();
                        return Err(e);
                    }
                }
            }

            // All retries exhausted.
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
        metadata: crate::api::value_object::GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcOutboundResult<GrpcMessageStream>> {
        // Streaming calls are not retried — partial state makes retry unsafe.
        self.inner.call_stream(method, metadata, messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::{GrpcMetadata, GrpcResponse, GrpcStatusCode};
    use futures::future::BoxFuture;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// Stub transport that fails the first `fail_count` calls with the given
    /// error code, then succeeds.
    struct FailThenSucceed {
        call_count: AtomicU32,
        fail_count: u32,
        fail_code:  GrpcStatusCode,
    }

    impl FailThenSucceed {
        fn new(fail_count: u32, fail_code: GrpcStatusCode) -> Arc<Self> {
            Arc::new(Self {
                call_count: AtomicU32::new(0),
                fail_count,
                fail_code,
            })
        }
    }

    impl GrpcOutbound for FailThenSucceed {
        fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
            let n = self.call_count.fetch_add(1, Ordering::SeqCst);
            if n < self.fail_count {
                let code = self.fail_code;
                Box::pin(futures::future::ready(Err(GrpcOutboundError::Status(
                    code, format!("fail #{n}"),
                ))))
            } else {
                Box::pin(futures::future::ready(Ok(GrpcResponse {
                    body:     vec![],
                    metadata: GrpcMetadata::default(),
                })))
            }
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    /// Always-fail transport.
    struct AlwaysFail(GrpcStatusCode);
    impl GrpcOutbound for AlwaysFail {
        fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
            let code = self.0;
            Box::pin(futures::future::ready(Err(GrpcOutboundError::Status(
                code, "always fail".into(),
            ))))
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    fn req() -> GrpcRequest {
        GrpcRequest::new("svc/Method", vec![], Duration::from_secs(5))
    }

    fn client_with(
        inner:       Arc<dyn GrpcOutbound>,
        max_attempts: u32,
        cb_threshold: u32,
    ) -> ResilientGrpcClient {
        ResilientGrpcClient {
            inner,
            retry: RetryPolicy {
                max_attempts,
                initial_backoff:    Duration::ZERO,
                backoff_multiplier: 1.0,
                max_backoff:        Duration::ZERO,
            },
            breaker: CircuitBreaker::new(cb_threshold, Duration::from_secs(60)),
        }
    }

    /// @covers: ResilientGrpcClient — succeeds on first attempt, no retry needed.
    #[tokio::test]
    async fn test_succeeds_on_first_attempt() {
        let inner = FailThenSucceed::new(0, GrpcStatusCode::Ok);
        let c = client_with(inner.clone(), 3, 10);
        assert!(c.call_unary(req()).await.is_ok());
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 1);
    }

    /// @covers: ResilientGrpcClient — retries RESOURCE_EXHAUSTED and succeeds.
    #[tokio::test]
    async fn test_retries_resource_exhausted_and_succeeds() {
        let inner = FailThenSucceed::new(2, GrpcStatusCode::ResourceExhausted);
        let c = client_with(inner.clone(), 3, 10);
        assert!(c.call_unary(req()).await.is_ok());
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 3);
    }

    /// @covers: ResilientGrpcClient — exhausts retries and returns last error.
    #[tokio::test]
    async fn test_exhausts_retries_and_returns_error() {
        let inner: Arc<dyn GrpcOutbound> = Arc::new(AlwaysFail(GrpcStatusCode::ResourceExhausted));
        let c = client_with(inner, 3, 10);
        let err = c.call_unary(req()).await.unwrap_err();
        assert!(matches!(
            err,
            GrpcOutboundError::Status(GrpcStatusCode::ResourceExhausted, _)
        ));
    }

    /// @covers: ResilientGrpcClient — non-retryable errors propagate immediately.
    #[tokio::test]
    async fn test_non_retryable_error_propagates_without_retry() {
        let inner = FailThenSucceed::new(1, GrpcStatusCode::Internal);
        let c = client_with(inner.clone(), 3, 10);
        assert!(c.call_unary(req()).await.is_err());
        assert_eq!(inner.call_count.load(Ordering::SeqCst), 1, "must not retry INTERNAL");
    }

    /// @covers: ResilientGrpcClient — circuit opens after threshold failures and
    /// subsequent calls fail fast without hitting the inner transport.
    #[tokio::test]
    async fn test_circuit_opens_and_fails_fast() {
        let inner: Arc<dyn GrpcOutbound> = Arc::new(AlwaysFail(GrpcStatusCode::Internal));
        let c = client_with(inner.clone(), 1, 2);

        // Two failures open the circuit.
        let _ = c.call_unary(req()).await;
        let _ = c.call_unary(req()).await;
        assert!(c.breaker.is_open());

        // Third call fails fast — inner is not contacted.
        let err = c.call_unary(req()).await.unwrap_err();
        assert!(
            matches!(err, GrpcOutboundError::Unavailable(ref m) if m.contains("circuit open")),
            "expected circuit-open Unavailable, got {err:?}"
        );
    }

    /// @covers: ResilientGrpcClient::health_check — delegates to inner.
    #[tokio::test]
    async fn test_health_check_delegates_to_inner() {
        let inner = FailThenSucceed::new(0, GrpcStatusCode::Ok);
        let c = client_with(inner, 1, 5);
        assert!(c.health_check().await.is_ok());
    }
}
