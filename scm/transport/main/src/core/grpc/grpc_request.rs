//! `impl` block for [`GrpcRequest`]. The type *declaration* lives in `api/`.

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use crate::api::{GrpcMetadata, GrpcRequest};

impl GrpcRequest {
    /// Create a new request for `method` with a raw protobuf `body` and a
    /// mandatory per-call `deadline`.
    ///
    /// `deadline` is a required positional argument — there is no overload
    /// without it and no default.  Compile error if omitted.
    pub fn new(method: impl Into<String>, body: Vec<u8>, deadline: Duration) -> Self {
        Self {
            method: method.into(),
            body,
            metadata: GrpcMetadata::default(),
            deadline,
            cancellation: None,
        }
    }

    /// Override the entire metadata block.
    pub fn with_metadata(mut self, metadata: GrpcMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Add a single metadata header key/value pair.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.headers.insert(name.into(), value.into());
        self
    }

    /// Attach a caller-supplied cancellation token.  Firing the token
    /// aborts the in-flight request from the caller's side.
    pub fn with_cancellation(mut self, token: CancellationToken) -> Self {
        self.cancellation = Some(token);
        self
    }
}
