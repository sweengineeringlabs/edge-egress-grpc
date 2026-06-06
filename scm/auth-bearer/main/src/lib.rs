//! `swe-edge-egress-grpc-auth-bearer` — JWT bearer
//! [`GrpcEgressInterceptor`](swe_edge_egress_grpc::GrpcEgressInterceptor)
//! for the gRPC egress stack.
//!
//! [`BearerEgressInterceptor`] signs a JWT from configured claims
//! (or accepts a pre-minted token) and injects it into the
//! `authorization` request metadata on every outgoing call.
//!
//! For the inbound (server-side) bearer validation counterpart see
//! `swe-edge-ingress-grpc-auth-bearer`.
//!
//! Constant-time comparisons (`subtle`) are used for any symmetric
//! shared-secret material in the configuration loaders.
//!
//! See `docs/threat_model.md` for the STRIDE breakdown.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]

mod api;
mod core;
mod saf;
mod spi;

mod gateway;
pub use gateway::*;
