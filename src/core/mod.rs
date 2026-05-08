//! gRPC core adapter implementations.
pub(crate) mod client;
pub(crate) mod interceptor;
pub(crate) mod resilience;
pub(crate) mod status_codes;

pub use client::{GrpcChannelConfigError, TonicGrpcClient};
pub use interceptor::TraceContextInterceptor;
pub use resilience::{
    classify_resource_exhausted, CircuitBreaker, ResilientGrpcClient,
    ResourceExhaustedContext, RetryDecision, RetryPolicy,
};
pub use status_codes::{from_tonic_code, from_wire, to_tonic_code, to_wire};
