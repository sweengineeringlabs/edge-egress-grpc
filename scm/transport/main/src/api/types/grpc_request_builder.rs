//! `GrpcRequestBuilder` — builder for [`crate::api::GrpcRequest`].

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use super::grpc_metadata::GrpcMetadata;

/// Builder for [`crate::api::GrpcRequest`].
#[derive(Debug, Default)]
pub struct GrpcRequestBuilder {
    pub(crate) method: Option<String>,
    pub(crate) body: Vec<u8>,
    pub(crate) deadline: Option<Duration>,
    pub(crate) metadata: GrpcMetadata,
    pub(crate) cancellation_token: Option<CancellationToken>,
}
