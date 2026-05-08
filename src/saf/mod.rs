//! SAF layer — gRPC public facade.

pub use crate::api::interceptor::{GrpcOutboundInterceptor, GrpcOutboundInterceptorChain};
pub use crate::api::port::{GrpcMessageStream, GrpcOutbound, GrpcOutboundError, GrpcOutboundResult};
pub use crate::api::value_object::{
    CompressionMode, GrpcChannelConfig, GrpcMetadata, GrpcRequest, GrpcResponse,
    GrpcStatusCode, KeepAliveConfig, MtlsConfig, DEFAULT_MAX_MESSAGE_BYTES,
};
pub use crate::api::value_object::ResilienceConfig;
pub use crate::core::{
    from_tonic_code, from_wire, to_tonic_code, to_wire, CircuitBreaker, GrpcChannelConfigError,
    ResilientGrpcClient, RetryPolicy, TonicGrpcClient, TraceContextInterceptor,
};
pub use crate::core::resilience::{
    classify_resource_exhausted, parse_retry_after_hint,
    ResourceExhaustedContext, RetryDecision,
};

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
/// Build an outbound transport from a [`GrpcChannelConfig`].
///
/// When `config.resilience` is `Some`, the bare [`TonicGrpcClient`] is
/// automatically wrapped in a [`ResilientGrpcClient`] with the configured
/// retry and circuit breaker policy — no manual wrapping required.
///
/// When `config.resilience` is `None`, the bare transport is returned.
///
/// # Errors
///
/// Returns [`GrpcChannelConfigError::PlaintextRejected`] when
/// `config.tls_required = true` and the endpoint scheme is `http://`.
pub fn create_transport_from_config(
    config: &crate::api::value_object::GrpcChannelConfig,
) -> Result<std::sync::Arc<dyn crate::api::port::GrpcOutbound>, GrpcChannelConfigError> {
    use std::time::Duration;

    let base: std::sync::Arc<dyn crate::api::port::GrpcOutbound> =
        std::sync::Arc::new(TonicGrpcClient::from_config(config)?);

    Ok(match &config.resilience {
        None => base,
        Some(r) => create_resilient_transport(
            base,
            RetryPolicy {
                max_attempts:                  r.max_attempts,
                initial_backoff:               Duration::from_millis(r.initial_backoff_ms),
                backoff_multiplier:            r.backoff_multiplier,
                max_backoff:                   Duration::from_millis(r.max_backoff_ms),
                rate_limit_max_attempts:       r.rate_limit_max_attempts,
                rate_limit_initial_backoff:    Duration::from_millis(r.rate_limit_initial_backoff_ms),
                rate_limit_max_backoff:        Duration::from_millis(r.rate_limit_max_backoff_ms),
            },
            r.failure_threshold,
            Duration::from_millis(r.open_duration_ms),
        ),
    })
}

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
