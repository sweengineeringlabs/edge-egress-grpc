//! `GrpcRequestBuilder` — builder for [`GrpcRequest`].

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use super::grpc_metadata::GrpcMetadata;
use super::grpc_request::GrpcRequest;

/// Builder for [`GrpcRequest`].
#[derive(Debug, Default)]
pub struct GrpcRequestBuilder {
    method: Option<String>,
    body: Vec<u8>,
    deadline: Option<Duration>,
    metadata: GrpcMetadata,
    cancellation_token: Option<CancellationToken>,
}

impl GrpcRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn method(mut self, v: impl Into<String>) -> Self {
        self.method = Some(v.into());
        self
    }
    pub fn body(mut self, v: Vec<u8>) -> Self {
        self.body = v;
        self
    }
    pub fn deadline(mut self, v: Duration) -> Self {
        self.deadline = Some(v);
        self
    }
    pub fn metadata(mut self, v: GrpcMetadata) -> Self {
        self.metadata = v;
        self
    }
    pub fn cancellation_token(mut self, v: CancellationToken) -> Self {
        self.cancellation_token = Some(v);
        self
    }

    /// Build the [`GrpcRequest`]. Returns `Err` when method or deadline is unset.
    pub fn build(self) -> Result<GrpcRequest, String> {
        let method = self.method.ok_or("method required")?;
        let deadline = self.deadline.ok_or("deadline required")?;
        let mut req = GrpcRequest::new(method, self.body, deadline);
        req.metadata = self.metadata;
        if let Some(token) = self.cancellation_token {
            req = req.with_cancellation(token);
        }
        Ok(req)
    }
}
