//! Scenario tests for `GrpcEgressProstExt::call_unary_typed` (ADR-026).
//!
//! Active only under `--features prost`. Exercise the typed unary helper over a
//! mock [`GrpcEgress`], proving the request is prost-encoded, the response is
//! prost-decoded, transport errors propagate, and an undecodable response
//! degrades to a client-side `Internal` error.
#![allow(clippy::unwrap_used, clippy::expect_used)]

#[cfg(feature = "prost")]
mod prost_tests {
    use std::time::Duration;

    use futures::future::BoxFuture;
    use prost::Message;

    use swe_edge_egress_grpc_transport::{
        GrpcEgress, GrpcEgressError, GrpcEgressProstExt, GrpcEgressResult, GrpcMetadata,
        GrpcRequest, GrpcResponse,
    };

    // ── prost message types under test ─────────────────────────────────────────

    #[derive(Clone, PartialEq, Message)]
    struct Ping {
        #[prost(uint32, tag = "1")]
        pub value: u32,
    }

    #[derive(Clone, PartialEq, Message)]
    struct Pong {
        #[prost(uint32, tag = "1")]
        pub value: u32,
    }

    // ── mock GrpcEgress impls ──────────────────────────────────────────────────

    /// Decodes the request as `Ping`, doubles it, returns an encoded `Pong`.
    /// Proves the helper both encodes the request and decodes the response.
    struct DoublingEgress;
    impl GrpcEgress for DoublingEgress {
        fn call_unary(
            &self,
            request: GrpcRequest,
        ) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
            Box::pin(async move {
                let ping = Ping::decode(request.body.as_slice())
                    .map_err(|e| GrpcEgressError::Internal(e.to_string()))?;
                let pong = Pong {
                    value: ping.value.wrapping_mul(2),
                };
                Ok(GrpcResponse {
                    body: pong.encode_to_vec(),
                    metadata: GrpcMetadata::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    /// Always fails at the transport level.
    struct UnavailableEgress;
    impl GrpcEgress for UnavailableEgress {
        fn call_unary(
            &self,
            _request: GrpcRequest,
        ) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
            Box::pin(async { Err(GrpcEgressError::Unavailable("remote down".into())) })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    /// Returns an undecodable response body (truncated varint).
    struct GarbageEgress;
    impl GrpcEgress for GarbageEgress {
        fn call_unary(
            &self,
            _request: GrpcRequest,
        ) -> BoxFuture<'_, GrpcEgressResult<GrpcResponse>> {
            Box::pin(async {
                Ok(GrpcResponse {
                    body: vec![0x08, 0x80],
                    metadata: GrpcMetadata::default(),
                })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    // ── _happy: typed request encoded, response decoded ────────────────────────

    /// @covers: GrpcEgressProstExt::call_unary_typed
    #[tokio::test]
    async fn test_call_unary_typed_roundtrips_prost_message_happy() {
        let client = DoublingEgress;

        let out: Pong = client
            .call_unary_typed(
                "/pkg.Echo/Double",
                &Ping { value: 21 },
                Duration::from_secs(5),
            )
            .await
            .unwrap();

        assert_eq!(
            out.value, 42,
            "request must be encoded and response decoded"
        );
    }

    // ── _error: transport error propagates unchanged ───────────────────────────

    /// @covers: GrpcEgressProstExt::call_unary_typed
    #[tokio::test]
    async fn test_call_unary_typed_propagates_transport_error() {
        let client = UnavailableEgress;

        let result: GrpcEgressResult<Pong> = client
            .call_unary_typed(
                "/pkg.Echo/Double",
                &Ping { value: 1 },
                Duration::from_secs(5),
            )
            .await;

        assert!(
            matches!(result, Err(GrpcEgressError::Unavailable(_))),
            "underlying transport error must propagate unchanged, got {result:?}"
        );
    }

    // ── _edge: undecodable response → Internal (client-side) ───────────────────

    /// @covers: GrpcEgressProstExt::call_unary_typed
    /// A response body that cannot be decoded is an unexpected client-side
    /// condition, mapped to `GrpcEgressError::Internal` — not silently dropped.
    #[tokio::test]
    async fn test_call_unary_typed_undecodable_response_maps_to_internal_edge() {
        let client = GarbageEgress;

        let result: GrpcEgressResult<Pong> = client
            .call_unary_typed(
                "/pkg.Echo/Double",
                &Ping { value: 1 },
                Duration::from_secs(5),
            )
            .await;

        match result {
            Err(GrpcEgressError::Internal(msg)) => {
                assert!(
                    msg.contains("decode"),
                    "internal error should explain the decode failure: {msg}"
                );
            }
            other => panic!("expected Internal decode error, got {other:?}"),
        }
    }
}
