//! `GrpcEgressProstCodec` — opt-in prost codec ergonomics for outbound gRPC (ADR-026).
//!
//! Feature-gated extension trait so the core [`GrpcEgress`] contract stays
//! byte-oriented and codec-free. Available only under the `prost` feature; the
//! default build gains no `prost` dependency.

use std::time::Duration;

use futures::future::BoxFuture;

use crate::api::{GrpcEgress, GrpcEgressError, GrpcEgressResult, GrpcRequest};

/// Typed unary calls over any [`GrpcEgress`], encoding and decoding via `prost`.
///
/// Blanket-implemented for every [`GrpcEgress`], so any client gains
/// [`call_unary_typed`](GrpcEgressProstCodec::call_unary_typed) with no extra
/// wiring when the `prost` feature is enabled.
pub trait GrpcEgressProstCodec: GrpcEgress {
    /// Encode `req` via prost, dispatch `method` with `deadline`, then decode the
    /// response body via prost.
    ///
    /// Transport- and status-level errors from the underlying
    /// [`call_unary`](GrpcEgress::call_unary) propagate unchanged. A response
    /// body that cannot be decoded is an unexpected client-side condition,
    /// mapped to [`GrpcEgressError::Internal`].
    fn call_unary_typed<Req, Resp>(
        &self,
        method: &str,
        req: &Req,
        deadline: Duration,
    ) -> BoxFuture<'_, GrpcEgressResult<Resp>>
    where
        Req: prost::Message,
        Resp: prost::Message + Default + 'static,
    {
        let request = GrpcRequest::new(method, req.encode_to_vec(), deadline);
        Box::pin(async move {
            let response = self.call_unary(request).await?;
            Resp::decode(response.body.as_slice())
                .map_err(|e| GrpcEgressError::Internal(format!("response decode failed: {e}")))
        })
    }
}

impl<T: GrpcEgress + ?Sized> GrpcEgressProstCodec for T {}
