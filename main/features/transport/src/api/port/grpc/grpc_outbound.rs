//! `GrpcOutbound` trait — makes outbound gRPC calls.

use futures::future::BoxFuture;

use crate::api::port::grpc::grpc_message_stream::GrpcMessageStream;
use crate::api::port::grpc::grpc_outbound_error::GrpcOutboundError;
use crate::api::port::grpc::grpc_outbound_result::GrpcOutboundResult;
use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode};

/// Makes outbound gRPC calls to remote services.
pub trait GrpcOutbound: Send + Sync {
    /// Send a single unary gRPC request and receive a single response.
    fn call_unary(&self, request: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>>;

    /// Send a streaming gRPC request and receive a response stream.
    ///
    /// The default implementation returns `Unimplemented` — override to enable streaming.
    fn call_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcOutboundResult<GrpcMessageStream>> {
        let _ = (method, metadata, messages);
        Box::pin(futures::future::ready(Err(GrpcOutboundError::Status(
            GrpcStatusCode::Unimplemented,
            "streaming not supported".into(),
        ))))
    }

    /// Check that the remote endpoint is reachable.
    fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::GrpcMetadata;
    use std::time::Duration;

    #[test]
    fn test_grpc_outbound_is_object_safe() {
        fn _assert_object_safe(_: &dyn GrpcOutbound) {}
    }

    #[tokio::test]
    async fn test_call_stream_default_returns_unimplemented_status() {
        struct UnaryOnlyClient;
        impl GrpcOutbound for UnaryOnlyClient {
            fn call_unary(
                &self,
                _: GrpcRequest,
            ) -> futures::future::BoxFuture<'_, GrpcOutboundResult<GrpcResponse>> {
                Box::pin(futures::future::ready(Ok(GrpcResponse {
                    body: vec![],
                    metadata: GrpcMetadata::default(),
                })))
            }
            fn health_check(&self) -> futures::future::BoxFuture<'_, GrpcOutboundResult<()>> {
                Box::pin(futures::future::ready(Ok(())))
            }
        }

        let _ = Duration::from_millis(1);
        let client = UnaryOnlyClient;
        let messages: GrpcMessageStream =
            Box::pin(futures::stream::empty::<GrpcOutboundResult<Vec<u8>>>());
        let result = client
            .call_stream("svc/Method".into(), GrpcMetadata::default(), messages)
            .await;
        match result {
            Err(GrpcOutboundError::Status(GrpcStatusCode::Unimplemented, msg)) => {
                assert!(
                    msg.contains("streaming not supported"),
                    "message was: {msg}"
                );
            }
            Ok(_) => panic!("expected Err(Status(Unimplemented)), got Ok"),
            Err(e) => panic!("expected Status(Unimplemented, _), got Err({e})"),
        }
    }
}
