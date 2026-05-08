//! SAF layer — gRPC public facade.

pub use crate::api::interceptor::{GrpcOutboundInterceptor, GrpcOutboundInterceptorChain};
pub use crate::api::port::{GrpcMessageStream, GrpcOutbound, GrpcOutboundError, GrpcOutboundResult};
pub use crate::api::value_object::{
    CompressionMode, GrpcChannelConfig, GrpcMetadata, GrpcRequest, GrpcResponse,
    GrpcStatusCode, KeepAliveConfig, MtlsConfig, DEFAULT_MAX_MESSAGE_BYTES,
};
pub use crate::core::{
    from_tonic_code, from_wire, to_tonic_code, to_wire, CircuitBreaker, GrpcChannelConfigError,
    ResilientGrpcClient, RetryPolicy, TonicGrpcClient, TraceContextInterceptor,
};
pub use crate::core::resilience::retry::{classify_resource_exhausted, ResourceExhaustedContext, RetryDecision};

/// Wrap `inner` with retry and circuit breaker behaviour.
///
/// The returned transport applies [`RetryPolicy`] on every `call_unary` —
/// retrying [`GrpcStatusCode::ResourceExhausted`] and
/// [`GrpcStatusCode::Unavailable`] with exponential backoff, bounded by
/// the caller's per-call deadline budget. After `failure_threshold`
/// consecutive failures that survive all retry attempts, the
/// [`CircuitBreaker`] opens and subsequent calls fail fast with
/// `Unavailable` until `open_duration` has elapsed and a probe succeeds.
///
/// # Example
///
/// ```ignore
/// let transport = create_resilient_transport(
///     create_grpc_transport("http://localhost:8082", deadline),
///     RetryPolicy { max_attempts: 3, ..Default::default() },
///     5,                           // open after 5 consecutive final failures
///     Duration::from_secs(10),     // stay open for 10 s before probing
/// );
/// ```
pub fn create_resilient_transport(
    inner:             std::sync::Arc<dyn crate::api::port::GrpcOutbound>,
    retry:             RetryPolicy,
    failure_threshold: u32,
    open_duration:     std::time::Duration,
) -> std::sync::Arc<dyn crate::api::port::GrpcOutbound> {
    std::sync::Arc::new(ResilientGrpcClient::new(
        inner,
        retry,
        CircuitBreaker::new(failure_threshold, open_duration),
    ))
}
