//! `GrpcEgress` trait — makes outbound gRPC calls.

use futures::future::BoxFuture;

use crate::api::error::GrpcEgressError;
use crate::api::types::GrpcEgressResult;
use crate::api::types::GrpcMessageStream;
use crate::api::types::{GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode};

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
