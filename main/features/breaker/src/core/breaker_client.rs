//! [`GrpcOutbound`] impl for [`GrpcBreakerClient`].
//!
//! - `call_unary`: admit → call inner → record outcome.  When
//!   admission is rejected, return `Unavailable` immediately
//!   without touching the inner client.
//! - `call_stream`: passes through.  Streaming has no clean
//!   single-outcome to record against the breaker; consumers
//!   that want streaming protection should layer their own
//!   per-message logic.
//! - `health_check`: passes through.  A health check is itself
//!   a probe; running it through the breaker would create a
//!   chicken-and-egg dance where the breaker's own state
//!   depends on a call the breaker is gating.

use futures::future::BoxFuture;
use swe_edge_egress_grpc::{
    GrpcMessageStream, GrpcMetadata, GrpcOutbound, GrpcOutboundError, GrpcOutboundResult,
    GrpcRequest, GrpcResponse,
};

use crate::api::breaker_client::GrpcBreakerClient;
use crate::api::breaker_state::Admission;
use crate::api::failure_kind::classify;
use crate::core::transitions::{admit, record};

impl<T: GrpcOutbound + Send + Sync + 'static> GrpcOutbound for GrpcBreakerClient<T> {
    fn call_unary(&self, request: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
        Box::pin(async move {
            // Admission decision under the lock.
            let decision = {
                let mut node = self.node.lock().await;
                admit(&mut node, &self.config)
            };

            match decision {
                Admission::RejectOpen => Err(GrpcOutboundError::Unavailable(
                    "grpc-breaker: circuit open, request short-circuited".into(),
                )),
                Admission::Proceed => {
                    let result = self.inner.call_unary(request).await;
                    let outcome = classify(&result);
                    {
                        let mut node = self.node.lock().await;
                        record(&mut node, &self.config, outcome);
                    }
                    result
                }
            }
        })
    }

    fn call_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcOutboundResult<GrpcMessageStream>> {
        self.inner.call_stream(method, metadata, messages)
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
        self.inner.health_check()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::future::BoxFuture;
    use swe_edge_egress_grpc::{
        GrpcMetadata, GrpcOutbound, GrpcOutboundResult, GrpcRequest, GrpcResponse,
    };

    use crate::api::breaker_client::GrpcBreakerClient;
    use crate::api::breaker_config::GrpcBreakerConfig;
    use crate::api::breaker_state::BreakerState;

    struct PongStub;
    impl GrpcOutbound for PongStub {
        fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
            Box::pin(async {
                Ok(GrpcResponse {
                    body: b"pong".to_vec(),
                    metadata: GrpcMetadata::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[tokio::test]
    async fn test_call_unary_passes_through_to_inner_when_closed() {
        let cfg = GrpcBreakerConfig::from_config(
            "failure_threshold = 3\ncool_down_seconds = 10\nhalf_open_probe_count = 1",
        )
        .unwrap();
        let client = GrpcBreakerClient::new(PongStub, cfg);
        let req = GrpcRequest::new("svc/M", b"ping".to_vec(), Duration::from_secs(1));
        let resp = client
            .call_unary(req)
            .await
            .expect("closed breaker must pass through to inner");
        assert_eq!(resp.body, b"pong");
        assert_eq!(client.state().await, BreakerState::Closed);
    }
}
