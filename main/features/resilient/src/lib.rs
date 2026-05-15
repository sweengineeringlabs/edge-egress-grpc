//! `swe_edge_egress_grpc_resilient` — assembled resilient gRPC transport.
//!
//! Provides [`create_resilient_transport_from_config`], which builds a
//! [`swe_edge_egress_grpc::TonicGrpcClient`] and, when
//! [`swe_edge_egress_grpc::GrpcChannelConfig::resilience`] is `Some`,
//! wraps it in a [`swe_edge_egress_grpc_retry::GrpcRetryClient`] then a
//! [`swe_edge_egress_grpc_breaker::GrpcBreakerClient`].
//!
//! Call-stack with resilience active:
//! ```text
//! GrpcBreakerClient   ← fast-fail when circuit is open
//!   └─ GrpcRetryClient ← exponential-backoff retry
//!        └─ TonicGrpcClient ← hyper HTTP/2 transport
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod api;
mod core;
mod saf;

pub use saf::*;

