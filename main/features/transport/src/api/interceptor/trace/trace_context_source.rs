//! `TraceContextSource` — selects the trace context injection mode.

/// Source of the trace context the interceptor will inject.
#[derive(Clone)]
pub enum TraceContextSource {
    /// No automatic injection — only propagates existing upstream value.
    PassThrough,
    /// Always inject a static `traceparent` (and optional `tracestate`).
    Static {
        traceparent: String,
        tracestate: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_through_variant_is_clone() {
        let s = TraceContextSource::PassThrough;
        let _ = s.clone();
    }

    #[test]
    fn test_static_variant_holds_traceparent_and_tracestate() {
        let s = TraceContextSource::Static {
            traceparent: "00-abc-def-01".into(),
            tracestate: Some("vendor=1".into()),
        };
        match s {
            TraceContextSource::Static {
                traceparent,
                tracestate,
            } => {
                assert_eq!(traceparent, "00-abc-def-01");
                assert_eq!(tracestate, Some("vendor=1".into()));
            }
            _ => panic!("expected Static"),
        }
    }
}
