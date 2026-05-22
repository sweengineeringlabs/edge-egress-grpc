//! W3C Trace Context outbound interceptor — declaration and implementation.

use crate::api::interceptor::grpc_egress::GrpcEgressInterceptor;
use crate::api::interceptor::TraceContextSource;
use crate::api::port::GrpcEgressError;
use crate::api::value_object::{GrpcRequest, GrpcResponse};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::{GrpcMetadata, GrpcRequest, GrpcResponse};
    use std::time::Duration;

    fn req() -> GrpcRequest {
        GrpcRequest::new("svc/M", vec![], Duration::from_secs(1))
    }

    /// @covers: pass_through
    #[test]
    fn test_pass_through_constructs_without_panic() {
        let _ = TraceContextInterceptor::pass_through();
    }

    /// @covers: with_static
    #[test]
    fn test_with_static_stores_traceparent() {
        let interceptor = TraceContextInterceptor::with_static("00-abc-01", None);
        match &interceptor.source {
            TraceContextSource::Static { traceparent, .. } => assert_eq!(traceparent, "00-abc-01"),
            _ => panic!("expected Static source"),
        }
    }

    /// @covers: pass_through
    #[test]
    fn test_pass_through_does_not_inject_traceparent_when_absent() {
        let ic = TraceContextInterceptor::pass_through();
        let mut r = req();
        ic.before_call(&mut r).unwrap();
        assert!(!r.metadata.headers.contains_key(TRACEPARENT));
    }

    /// @covers: with_static
    #[test]
    fn test_with_static_injects_traceparent_when_absent() {
        let tp = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let ic = TraceContextInterceptor::with_static(tp, None);
        let mut r = req();
        ic.before_call(&mut r).unwrap();
        assert_eq!(
            r.metadata.headers.get(TRACEPARENT).map(String::as_str),
            Some(tp)
        );
    }

    /// @covers: with_static
    #[test]
    fn test_with_static_injects_tracestate_when_configured() {
        let tp = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let ic = TraceContextInterceptor::with_static(tp, Some("v=1".into()));
        let mut r = req();
        ic.before_call(&mut r).unwrap();
        assert_eq!(
            r.metadata.headers.get(TRACESTATE).map(String::as_str),
            Some("v=1")
        );
    }

    /// @covers: with_static
    #[test]
    fn test_upstream_traceparent_is_preserved_and_not_overwritten() {
        let upstream = "00-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa1-bbbbbbbbbbbbbbbb-01";
        let injected = "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01";
        let ic = TraceContextInterceptor::with_static(injected, None);
        let mut r = req();
        r.metadata
            .headers
            .insert(TRACEPARENT.into(), upstream.into());
        ic.before_call(&mut r).unwrap();
        assert_eq!(
            r.metadata.headers.get(TRACEPARENT).map(String::as_str),
            Some(upstream)
        );
    }

    /// @covers: pass_through
    #[test]
    fn test_after_call_does_not_modify_response() {
        let ic = TraceContextInterceptor::pass_through();
        let mut resp = GrpcResponse {
            body: vec![],
            metadata: GrpcMetadata::default(),
        };
        ic.after_call(&mut resp).unwrap();
        assert!(resp.metadata.headers.is_empty());
    }
}
