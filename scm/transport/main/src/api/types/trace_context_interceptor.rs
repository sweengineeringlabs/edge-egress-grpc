//! W3C Trace Context outbound interceptor — value object declaration.

use crate::api::types::trace_context_source::TraceContextSource;

/// W3C Trace Context propagation interceptor for outbound gRPC.
///
/// Construct via [`TraceContextInterceptor::pass_through`] or
/// [`TraceContextInterceptor::with_static`].
#[derive(Clone)]
pub struct TraceContextInterceptor {
    pub(crate) source: TraceContextSource,
}
