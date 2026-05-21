//! Primary trait re-export hub for `swe_edge_egress_grpc_breaker`.
//!
//! This crate's primary trait is
//! [`GrpcOutbound`](swe_edge_egress_grpc::GrpcOutbound) — declared
//! upstream in `swe-edge-egress-grpc`. Consumers of this crate should
//! depend on `swe-edge-egress-grpc` directly for the trait — this
//! crate's job is to wrap implementors, not to re-publish the contract.

#[cfg(test)]
mod tests {
    use futures::future::BoxFuture;
    use swe_edge_egress_grpc::{
        GrpcMetadata, GrpcOutbound, GrpcOutboundResult, GrpcRequest, GrpcResponse,
    };

    #[tokio::test]
    async fn test_grpc_outbound_re_export_is_reachable_as_trait_bound() {
        struct HealthyStub;
        impl GrpcOutbound for HealthyStub {
            fn call_unary(
                &self,
                _: GrpcRequest,
            ) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
                Box::pin(async {
                    Ok(GrpcResponse {
                        body: vec![],
                        metadata: GrpcMetadata::default(),
                    })
                })
            }
            fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>> {
                Box::pin(async { Ok(()) })
            }
        }

        let result = HealthyStub.health_check().await;
        assert!(result.is_ok(), "health check on stub must succeed");
    }
}
