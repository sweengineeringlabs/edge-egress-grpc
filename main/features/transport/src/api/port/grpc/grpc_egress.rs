//! `GrpcEgress` trait — makes outbound gRPC calls.

use futures::future::BoxFuture;

use crate::api::port::grpc::grpc_egress_error::GrpcEgressError;
use crate::api::port::grpc::grpc_egress_result::GrpcEgressResult;
use crate::api::port::grpc::grpc_message_stream::GrpcMessageStream;
use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode};

/// Makes outbound gRPC calls to remote services.
pub trait GrpcEgress: Send + Sync {
    /// Send a single unary gRPC request and receive a single response.
    fn call_unary(&self, request: GrpcRequest) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>>;

    /// Send a streaming gRPC request and receive a response stream.
    ///
    /// The default implementation returns `Unimplemented` — override to enable streaming.
    fn call_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcMessageStream>> {
        let _ = (method, metadata, messages);
        Box::pin(futures::future::ready(Err(GrpcEgressError::Status(
            GrpcStatusCode::Unimplemented,
            "streaming not supported".into(),
        ))))
    }

    /// Send a server-streaming request — single request, streaming response.
    ///
    /// The default implementation returns `Unimplemented` — override to enable
    /// server-streaming.
    fn call_server_stream(
        &self,
        request: GrpcRequest,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcMessageStream>> {
        let _ = request;
        Box::pin(futures::future::ready(Err(GrpcEgressError::Status(
            GrpcStatusCode::Unimplemented,
            "server streaming not supported".into(),
        ))))
    }

    /// Send a client-streaming request — streaming request messages, single response.
    ///
    /// The default implementation returns `Unimplemented` — override to enable
    /// client-streaming.
    fn call_client_stream(
        &self,
        method: String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
    ) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
        let _ = (method, metadata, messages);
        Box::pin(futures::future::ready(Err(GrpcEgressError::Status(
            GrpcStatusCode::Unimplemented,
            "client streaming not supported".into(),
        ))))
    }

    /// Send a bidirectional-streaming request — streaming in both directions.
    ///
    /// The default implementation delegates to [`call_stream`].
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

    /// Check that the remote endpoint is reachable.
    fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::GrpcMetadata;
    use std::time::Duration;

    struct UnaryOnlyClient;
    impl GrpcEgress for UnaryOnlyClient {
        fn call_unary(
            &self,
            _: GrpcRequest,
        ) -> futures::future::BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
            Box::pin(futures::future::ready(Ok(GrpcResponse {
                body: vec![],
                metadata: GrpcMetadata::default(),
            })))
        }
        fn health_check(&self) -> futures::future::BoxFuture<'_, GrpcEgressResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    #[test]
    fn test_grpc_egress_is_object_safe() {
        fn _assert_object_safe(_: &dyn GrpcEgress) {}
    }

    #[tokio::test]
    async fn test_call_stream_default_returns_unimplemented_status() {
        let _ = Duration::from_millis(1);
        let client = UnaryOnlyClient;
        let messages: GrpcMessageStream =
            Box::pin(futures::stream::empty::<GrpcEgressResult<Vec<u8>>>());
        let result = client
            .call_stream("svc/Method".into(), GrpcMetadata::default(), messages)
            .await;
        match result {
            Err(GrpcEgressError::Status(GrpcStatusCode::Unimplemented, msg)) => {
                assert!(
                    msg.contains("streaming not supported"),
                    "message was: {msg}"
                );
            }
            Ok(_) => panic!("expected Err(Status(Unimplemented)), got Ok"),
            Err(e) => panic!("expected Status(Unimplemented, _), got Err({e})"),
        }
    }

    #[tokio::test]
    async fn test_call_server_stream_default_returns_unimplemented() {
        let client = UnaryOnlyClient;
        let req = GrpcRequest::new("svc/M".to_string(), vec![], Duration::from_secs(1));
        let result = client.call_server_stream(req).await;
        assert!(matches!(
            result,
            Err(GrpcEgressError::Status(GrpcStatusCode::Unimplemented, _))
        ));
    }

    #[tokio::test]
    async fn test_call_client_stream_default_returns_unimplemented() {
        use futures::stream;
        let client = UnaryOnlyClient;
        let messages: GrpcMessageStream = Box::pin(stream::empty::<GrpcEgressResult<Vec<u8>>>());
        let result = client
            .call_client_stream("svc/M".into(), GrpcMetadata::default(), messages)
            .await;
        assert!(matches!(
            result,
            Err(GrpcEgressError::Status(GrpcStatusCode::Unimplemented, _))
        ));
    }

    #[tokio::test]
    async fn test_call_bidi_stream_default_delegates_to_call_stream() {
        use futures::stream;
        let client = UnaryOnlyClient;
        let messages: GrpcMessageStream = Box::pin(stream::empty::<GrpcEgressResult<Vec<u8>>>());
        let result = client
            .call_bidi_stream("svc/M".into(), GrpcMetadata::default(), messages)
            .await;
        match result {
            Err(GrpcEgressError::Status(GrpcStatusCode::Unimplemented, _)) => {}
            Ok(_) => panic!("expected Unimplemented"),
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}
