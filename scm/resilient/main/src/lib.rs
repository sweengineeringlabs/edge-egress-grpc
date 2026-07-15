//! `edge_transport_grpc_egress_resilient` — assembled resilient gRPC transport.
//!
//! Use [`GrpcResilientFacade::create_resilient_transport_from_config`] to build a
//! [`edge_transport_grpc_egress::TonicGrpcClient`] and, when
//! [`edge_transport_grpc_egress::GrpcChannelConfig::resilience`] is `Some`,
//! wraps it in a [`edge_transport_grpc_egress_retry::GrpcRetryClient`] then a
//! [`edge_transport_grpc_egress_breaker::GrpcBreakerClient`].
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
    GrpcResilientSvcProcessor, Processor, ResilienceConfig, ResilientTransportError, Validator,
};
pub use saf::*;
