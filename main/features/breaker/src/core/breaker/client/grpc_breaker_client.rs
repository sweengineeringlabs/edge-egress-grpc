//! [`GrpcEgress`] impl for [`GrpcBreakerClient`].
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
    GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcMessageStream, GrpcMetadata, GrpcRequest,
    GrpcResponse,
};

use crate::api::breaker::admission::Admission;
use crate::api::breaker::failure_kind::classify;
use crate::api::breaker::grpc_breaker_client::GrpcBreakerClient;
use crate::core::transitions::admit;
use crate::core::transitions::record;

impl<T: GrpcEgress + Send + Sync + 'static> GrpcEgress for GrpcBreakerClient<T> {
    fn call_unary(&self, request: GrpcRequest) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
        Box::pin(async move {
            // Admission decision under the lock.
            let decision = {
                let mut node = self.node.lock().await;
                admit(&mut node, &self.config)
            };

            match decision {
                Admission::RejectOpen => Err(GrpcEgressError::Unavailable(
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
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcMessageStream>> {
        self.inner.call_stream(method, metadata, messages)
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
        self.inner.health_check()
    }
}
