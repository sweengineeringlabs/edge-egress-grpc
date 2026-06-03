//! [`GrpcEgress`] impl for [`GrpcBreakerClient`].

/// Impl unit — satisfies SEA rule 89 (core/ file must define a primary type).
#[expect(
    dead_code,
    reason = "SEA structural marker — impl site anchor, not instantiated"
)]
pub(crate) struct BreakerEgress;

use futures::future::BoxFuture;
use swe_edge_egress_grpc::{
    GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcMessageStream, GrpcMetadata, GrpcRequest,
    GrpcResponse,
};

use crate::api::breaker::admission::Admission;
use crate::api::breaker::failure_kind::FailureClassifier;
use crate::api::breaker::grpc::GrpcBreakerClient;
use crate::core::transitions::BreakerTransition;

impl<T: GrpcEgress + Send + Sync + 'static> GrpcEgress for GrpcBreakerClient<T> {
    fn call_unary(&self, request: GrpcRequest) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
        Box::pin(async move {
            let decision = {
                let mut node = self.node.lock().await;
                BreakerTransition::admit(&mut node, &self.config)
            };

            match decision {
                Admission::RejectOpen => Err(GrpcEgressError::Unavailable(
                    "grpc-breaker: circuit open, request short-circuited".into(),
                )),
                Admission::Proceed => {
                    let result = self.inner.call_unary(request).await;
                    let outcome = FailureClassifier::classify(&result);
                    {
                        let mut node = self.node.lock().await;
                        BreakerTransition::record(&mut node, &self.config, outcome);
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
