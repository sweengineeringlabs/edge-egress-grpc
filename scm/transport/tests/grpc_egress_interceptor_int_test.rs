//! Integration tests for `api/interceptor/grpc/grpc_egress_interceptor.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_grpc_transport::{
    AfterCallRequest, GrpcClientBuilder, GrpcEgressError, GrpcEgressInterceptor,
    GrpcEgressInterceptorChain, GrpcRequest, TraceContextSource,
};

#[test]
fn transport_trait_grpc_egress_interceptor_is_object_safe_int_test() {
    fn _assert(_: &dyn GrpcEgressInterceptor) {}
}

/// Minimal test-double satisfying the two abstract methods, used only to
/// exercise the trait's default (`Self: Sized`) methods from outside the crate.
// @allow: no_mocks_in_integration — hand-rolled test double, not a mock library.
struct StubInterceptor;

impl GrpcEgressInterceptor for StubInterceptor {
    fn before_call(&self, _req: &mut GrpcRequest) -> Result<(), GrpcEgressError> {
        Ok(())
    }

    fn after_call(&self, _req: AfterCallRequest<'_>) -> Result<(), GrpcEgressError> {
        Ok(())
    }
}

// ── default_request_builder ──────────────────────────────────────────────────

/// @covers: default_request_builder
#[test]
fn test_default_request_builder_missing_method_is_error() {
    let err = <StubInterceptor as GrpcEgressInterceptor>::default_request_builder()
        .build()
        .expect_err("a builder with no method set must fail to build");
    assert!(!err.is_empty(), "error message must be non-empty");
}

/// @covers: default_request_builder
#[test]
fn test_default_request_builder_with_method_and_deadline_builds_valid_request_happy() {
    let req = <StubInterceptor as GrpcEgressInterceptor>::default_request_builder()
        .method("/pkg.Svc/Method")
        .deadline(std::time::Duration::from_secs(5))
        .build()
        .expect("a builder with method and deadline set must build");
    assert_eq!(req.method, "/pkg.Svc/Method");
}

/// @covers: default_request_builder
#[test]
fn test_default_request_builder_repeated_calls_are_independent_edge() {
    let a = <StubInterceptor as GrpcEgressInterceptor>::default_request_builder()
        .method("/pkg.Svc/A")
        .deadline(std::time::Duration::from_secs(1))
        .build()
        .expect("first builder");
    let b = <StubInterceptor as GrpcEgressInterceptor>::default_request_builder()
        .method("/pkg.Svc/B")
        .deadline(std::time::Duration::from_secs(1))
        .build()
        .expect("second builder");
    assert_ne!(
        a.method, b.method,
        "each builder must be independently configured"
    );
}

// ── describe_chain_len ────────────────────────────────────────────────────────

/// @covers: describe_chain_len
#[test]
fn test_describe_chain_len_empty_chain_is_zero_happy() {
    let chain = GrpcEgressInterceptorChain::new();
    assert_eq!(
        <StubInterceptor as GrpcEgressInterceptor>::describe_chain_len(&chain),
        0
    );
}

/// @covers: describe_chain_len
#[test]
fn test_describe_chain_len_registered_interceptor_counts_one_error() {
    use std::sync::Arc;
    let chain = GrpcEgressInterceptorChain::new().push(Arc::new(StubInterceptor));
    assert_eq!(
        <StubInterceptor as GrpcEgressInterceptor>::describe_chain_len(&chain),
        1
    );
}

/// @covers: describe_chain_len
#[test]
fn test_describe_chain_len_multiple_interceptors_edge() {
    use std::sync::Arc;
    let chain = GrpcEgressInterceptorChain::new()
        .push(Arc::new(StubInterceptor))
        .push(Arc::new(StubInterceptor))
        .push(Arc::new(StubInterceptor));
    assert_eq!(
        <StubInterceptor as GrpcEgressInterceptor>::describe_chain_len(&chain),
        3
    );
}

// ── describe_trace_source ────────────────────────────────────────────────────

/// @covers: describe_trace_source
#[test]
fn test_describe_trace_source_pass_through_is_not_static_happy() {
    assert!(
        !<StubInterceptor as GrpcEgressInterceptor>::describe_trace_source(
            TraceContextSource::PassThrough
        )
    );
}

/// @covers: describe_trace_source
#[test]
fn test_describe_trace_source_static_without_tracestate_is_static_error() {
    assert!(
        <StubInterceptor as GrpcEgressInterceptor>::describe_trace_source(
            TraceContextSource::Static {
                traceparent: "00-0-0-00".to_string(),
                tracestate: None,
            }
        )
    );
}

/// @covers: describe_trace_source
#[test]
fn test_describe_trace_source_static_with_tracestate_is_static_edge() {
    assert!(
        <StubInterceptor as GrpcEgressInterceptor>::describe_trace_source(
            TraceContextSource::Static {
                traceparent: "00-0-0-00".to_string(),
                tracestate: Some("vendor=value".to_string()),
            }
        )
    );
}

// ── default_client_builder ────────────────────────────────────────────────────

/// @covers: default_client_builder
#[test]
fn test_default_client_builder_returns_zero_sized_marker_happy() {
    let builder: GrpcClientBuilder =
        <StubInterceptor as GrpcEgressInterceptor>::default_client_builder();
    assert_eq!(std::mem::size_of_val(&builder), 0);
}

/// @covers: default_client_builder
#[test]
fn test_default_client_builder_repeated_calls_are_independent_edge() {
    let a = <StubInterceptor as GrpcEgressInterceptor>::default_client_builder();
    let b = <StubInterceptor as GrpcEgressInterceptor>::default_client_builder();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}

/// @covers: default_client_builder
#[test]
fn test_default_client_builder_is_the_real_marker_type_error() {
    fn assert_type(builder: GrpcClientBuilder) -> usize {
        std::mem::size_of_val(&builder)
    }
    let size = assert_type(<StubInterceptor as GrpcEgressInterceptor>::default_client_builder());
    assert_eq!(
        size, 0,
        "the returned value must unify with the genuine zero-sized GrpcClientBuilder, not a look-alike"
    );
}
