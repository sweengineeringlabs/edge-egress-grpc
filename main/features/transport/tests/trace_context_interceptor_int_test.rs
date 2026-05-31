//! Integration tests for `TraceContextInterceptor`.
//!
//! Tests that access the `pub(crate)` field `interceptor.source` are omitted.

use std::time::Duration;
use swe_edge_egress_grpc_transport::GrpcEgressInterceptor;
use swe_edge_egress_grpc_transport::{
    GrpcMetadata, GrpcRequest, GrpcResponse, TraceContextInterceptor,
};

fn req() -> GrpcRequest {
    GrpcRequest::new("svc/M", vec![], Duration::from_secs(1))
}

/// @covers: TraceContextInterceptor::pass_through — constructs without panic
#[test]
fn transport_struct_trace_context_interceptor_pass_through_constructs_without_panic_int_test() {
    let _ = TraceContextInterceptor::pass_through();
}

/// @covers: TraceContextInterceptor::pass_through — does not inject traceparent when absent
#[test]
fn transport_struct_trace_context_interceptor_pass_through_does_not_inject_traceparent_int_test() {
    let ic = TraceContextInterceptor::pass_through();
    let mut r = req();
    ic.before_call(&mut r).expect("before_call must not fail");
    assert!(
        !r.metadata.headers.contains_key("traceparent"),
        "pass_through must not inject a traceparent header"
    );
}

/// @covers: TraceContextInterceptor::with_static — injects traceparent when absent
#[test]
fn transport_struct_trace_context_interceptor_with_static_injects_traceparent_when_absent_int_test()
{
    let tp = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
    let ic = TraceContextInterceptor::with_static(tp, None);
    let mut r = req();
    ic.before_call(&mut r).expect("before_call must not fail");
    assert_eq!(
        r.metadata.headers.get("traceparent").map(String::as_str),
        Some(tp),
        "with_static must inject traceparent when absent"
    );
}

/// @covers: TraceContextInterceptor::with_static — injects tracestate when configured
#[test]
fn transport_struct_trace_context_interceptor_with_static_injects_tracestate_when_configured_int_test(
) {
    let tp = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
    let ic = TraceContextInterceptor::with_static(tp, Some("v=1".into()));
    let mut r = req();
    ic.before_call(&mut r).expect("before_call must not fail");
    assert_eq!(
        r.metadata.headers.get("tracestate").map(String::as_str),
        Some("v=1"),
        "with_static must inject tracestate when configured"
    );
}

/// @covers: TraceContextInterceptor::with_static — upstream traceparent is preserved
#[test]
fn transport_struct_trace_context_interceptor_upstream_traceparent_is_not_overwritten_int_test() {
    let upstream = "00-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1-bbbbbbbbbbbbbbbb-01";
    let injected = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
    let ic = TraceContextInterceptor::with_static(injected, None);
    let mut r = req();
    r.metadata
        .headers
        .insert("traceparent".into(), upstream.into());
    ic.before_call(&mut r).expect("before_call must not fail");
    assert_eq!(
        r.metadata.headers.get("traceparent").map(String::as_str),
        Some(upstream),
        "existing traceparent must not be overwritten"
    );
}

/// @covers: TraceContextInterceptor::after_call — does not modify response
#[test]
fn transport_struct_trace_context_interceptor_after_call_does_not_modify_response_int_test() {
    let ic = TraceContextInterceptor::pass_through();
    let mut resp = GrpcResponse {
        body: vec![],
        metadata: GrpcMetadata::default(),
    };
    ic.after_call(&mut resp).expect("after_call must not fail");
    assert!(resp.metadata.headers.is_empty());
}
