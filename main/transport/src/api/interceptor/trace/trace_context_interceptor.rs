//! W3C Trace Context outbound interceptor — declaration and implementation.

use crate::api::interceptor::grpc::GrpcEgressInterceptor;
use crate::api::interceptor::TraceContextSource;
use crate::api::error::GrpcEgressError;
use crate::api::value::{GrpcRequest, GrpcResponse};

const TRACEPARENT: &str = "traceparent";
const TRACESTATE: &str = "tracestate";

/// W3C Trace Context propagation interceptor for outbound gRPC.
///
/// Construct via [`TraceContextInterceptor::pass_through`] or
/// [`TraceContextInterceptor::with_static`].
#[derive(Clone)]
pub struct TraceContextInterceptor {
    pub(crate) source: TraceContextSource,
}

impl TraceContextInterceptor {
    /// Propagates an upstream `traceparent` only — does not inject.
    pub fn pass_through() -> Self {
        Self {
            source: TraceContextSource::PassThrough,
        }
    }

    /// Inject a fixed `traceparent` when none is already present.
    pub fn with_static(traceparent: impl Into<String>, tracestate: Option<String>) -> Self {
        Self {
            source: TraceContextSource::Static {
                traceparent: traceparent.into(),
                tracestate,
            },
        }
    }
}

impl GrpcEgressInterceptor for TraceContextInterceptor {
    fn before_call(&self, req: &mut GrpcRequest) -> Result<(), GrpcEgressError> {
        if req.metadata.headers.contains_key(TRACEPARENT) {
            return Ok(());
        }
        match &self.source {
            TraceContextSource::PassThrough => Ok(()),
            TraceContextSource::Static {
                traceparent,
                tracestate,
            } => {
                req.metadata
                    .headers
                    .insert(TRACEPARENT.into(), traceparent.clone());
                if let Some(state) = tracestate {
                    req.metadata
                        .headers
                        .insert(TRACESTATE.into(), state.clone());
                }
                Ok(())
            }
        }
    }

    fn after_call(&self, _resp: &mut GrpcResponse) -> Result<(), GrpcEgressError> {
        Ok(())
    }
}
