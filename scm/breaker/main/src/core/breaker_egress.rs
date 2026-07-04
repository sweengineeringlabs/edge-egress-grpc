//! [`GrpcEgress`] impl for [`GrpcBreakerClient`].

use futures::future::BoxFuture;
use swe_edge_egress_grpc::{
    GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcMessageStream, GrpcMetadata, GrpcRequest,
    GrpcResponse,
};

use crate::api::{
    Admission, AdmitRequest, BreakerTransition, GrpcBreakerClient, RecordOutcomeRequest,
};
use crate::core::breaker_transition::DefaultBreakerTransition;
use crate::core::failure_classifier::FailureClassifier;

impl<T: GrpcEgress + Send + Sync + 'static> GrpcEgress for GrpcBreakerClient<T> {
    fn call_unary(&self, request: GrpcRequest) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
        Box::pin(async move {
            let node_snapshot = *self.node.lock().await;
            let admit_resp = match DefaultBreakerTransition.admit(AdmitRequest {
                node: node_snapshot,
                config: (*self.config).clone(),
            }) {
                Ok(resp) => resp,
                // BreakerDomainError is never actually constructed by admit();
                // this branch exists only because the trait's signature is
                // honest about the general Result contract.
                Err(e) => {
                    return Err(GrpcEgressError::Internal(format!(
                        "grpc-breaker: admit failed unexpectedly: {e}"
                    )))
                }
            };
            *self.node.lock().await = admit_resp.node;

            match admit_resp.admission {
                Admission::RejectOpen => Err(GrpcEgressError::Unavailable(
                    "grpc-breaker: circuit open, request short-circuited".into(),
                )),
                Admission::Proceed => {
                    let result = self.inner.call_unary(request).await;
                    let outcome = FailureClassifier::classify(&result);
                    let node_snapshot = *self.node.lock().await;
                    let record_resp = match DefaultBreakerTransition.record(RecordOutcomeRequest {
                        node: node_snapshot,
                        config: (*self.config).clone(),
                        outcome,
                    }) {
                        Ok(resp) => resp,
                        Err(e) => {
                            return Err(GrpcEgressError::Internal(format!(
                                "grpc-breaker: record failed unexpectedly: {e}"
                            )))
                        }
                    };
                    *self.node.lock().await = record_resp.node;
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
