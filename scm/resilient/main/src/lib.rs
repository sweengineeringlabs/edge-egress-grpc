//! `swe_edge_egress_grpc_resilient` — assembled resilient gRPC transport.
//!
//! Use [`GrpcResilientFacade::create_resilient_transport_from_config`] to build a
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
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

// Public contracts and value objects — all flow directly from api/.
pub use crate::api::{
    ApplicationConfigBuilder, ConfigBuilderProvider, ConfigBuilderRequest, ConfigBuilderResponse,
    ConfigValidationRequest, DescribeRequest, DescribeResponse, GrpcResilientFacade,
    GrpcResilientSvc, Processor, ResilienceConfig, ResilientTransportError, Validator,
};
pub use saf::*;
