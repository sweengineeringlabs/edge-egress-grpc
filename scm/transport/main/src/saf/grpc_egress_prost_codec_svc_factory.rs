//! Composition site for [`GrpcEgressProstCodec`] — one file per trait keeps wiring focused.
//!
//! Also hosts [`TransportSvc::call_unary_typed`], the generic prost
//! ergonomics helper. It lives here (not on the trait itself) because a
//! generic method can't be part of an object-safe, api/-declared port
//! contract — see the trait's own doc comment in `api/traits/`. saf/ has no
//! such restriction, so the actual encode/dispatch/decode convenience is
//! implemented here instead, calling through the trait's byte-oriented
//! `call_unary_encoded` method.
//!
//! `GrpcEgressProstCodec` itself is implemented per concrete `GrpcEgress`
//! type (in each type's own file, alongside its `GrpcEgress` impl) rather
//! than via one blanket `impl<T: GrpcEgress + ?Sized>`: a blanket impl has
//! no concrete-type target, which `core_api_module_correspondence` doesn't
//! recognize as a genuine trait-implementor site, and `core_implements_api_traits`
//! separately requires at least one concrete `impl Trait for Struct` to exist
//! in core/ — both rules are satisfied by concrete per-type impls, verified
//! empirically after a blanket impl (wherever placed) failed one or the other.

use std::time::Duration;

pub use crate::api::GrpcEgressProstCodec;
use crate::api::{GrpcEgressError, GrpcEgressResult, GrpcRequest, TransportSvc};

impl TransportSvc {
    /// Encode `req` via prost, dispatch `method` on `client` with `deadline`,
    /// then decode the response body via prost.
    ///
    /// Transport- and status-level errors from the underlying
    /// [`call_unary_encoded`](GrpcEgressProstCodec::call_unary_encoded) propagate
    /// unchanged. A response body that cannot be decoded is an unexpected
    /// client-side condition, mapped to [`GrpcEgressError::Internal`].
    pub async fn call_unary_typed<C, Req, Resp>(
        client: &C,
        method: &str,
        req: &Req,
        deadline: Duration,
    ) -> GrpcEgressResult<Resp>
    where
        C: GrpcEgressProstCodec + ?Sized,
        Req: prost::Message,
        Resp: prost::Message + Default + 'static,
    {
        let request = GrpcRequest::new(method, req.encode_to_vec(), deadline);
        let response = client.call_unary_encoded(request).await?;
        Resp::decode(response.body.as_slice())
            .map_err(|e| GrpcEgressError::Internal(format!("response decode failed: {e}")))
    }
}
