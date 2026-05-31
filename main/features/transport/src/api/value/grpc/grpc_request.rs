//! gRPC request envelope.

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use super::grpc_metadata::GrpcMetadata;

/// A gRPC request envelope.
///
/// ## Mandatory deadline
///
/// Every gRPC call MUST carry a per-call deadline.  Production gRPC
/// without deadlines deadlocks on slow servers or stuck connections —
/// the deadline is therefore a *required* positional argument of
/// [`GrpcRequest::new`], not an `Option`.  Construction without a
/// `Duration` is a compile error.
///
/// ## Optional cancellation
///
/// A caller-supplied [`CancellationToken`] can be attached via
/// [`GrpcRequest::with_cancellation`].  When the token fires, the
/// in-flight request is aborted by the egress transport.  Default:
/// no token (caller never cancels).
#[derive(Debug, Clone)]
pub struct GrpcRequest {
    /// Fully-qualified gRPC method path (e.g. `"pkg.Service/Method"`).
    pub method: String,
    /// Raw protobuf-encoded request bytes.
    pub body: Vec<u8>,
    /// Request metadata (headers).
    pub metadata: GrpcMetadata,
    /// Per-call deadline.  Always present.
    pub deadline: Duration,
    /// Optional caller-supplied cancellation token.
    pub cancellation: Option<CancellationToken>,
}

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
