//! `GrpcOutboundInterceptorChain` — ordered chain of outbound interceptors.

use std::sync::Arc;

use crate::api::interceptor::GrpcOutboundInterceptor;
use crate::api::port::GrpcOutboundError;
use crate::api::value_object::{GrpcRequest, GrpcResponse};

/// A registered chain of [`GrpcOutboundInterceptor`]s.
///
/// Chain order = the order in which interceptors were added.
#[derive(Clone, Default)]
pub struct GrpcOutboundInterceptorChain {
    pub(crate) interceptors: Vec<Arc<dyn GrpcOutboundInterceptor>>,
}

impl GrpcOutboundInterceptorChain {
    /// Construct an empty chain.
    pub fn new() -> Self {
        Self {
            interceptors: Vec::new(),
        }
    }

    /// Register `interceptor` at the end of the chain.
    pub fn push(mut self, interceptor: Arc<dyn GrpcOutboundInterceptor>) -> Self {
        self.interceptors.push(interceptor);
        self
    }

    /// Number of registered interceptors.
    pub fn len(&self) -> usize {
        self.interceptors.len()
    }

    /// `true` when no interceptors are registered.
    pub fn is_empty(&self) -> bool {
        self.interceptors.is_empty()
    }

    /// Run every `before_call` in order until one fails or all succeed.
    pub fn run_before(&self, req: &mut GrpcRequest) -> Result<(), GrpcOutboundError> {
        for interceptor in &self.interceptors {
            interceptor.before_call(req)?;
        }
        Ok(())
    }

    /// Run every `after_call` in order until one fails or all succeed.
    pub fn run_after(&self, resp: &mut GrpcResponse) -> Result<(), GrpcOutboundError> {
        for interceptor in &self.interceptors {
            interceptor.after_call(resp)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;
    use std::time::Duration;

    use crate::api::value_object::{GrpcMetadata, GrpcStatusCode};

    use super::*;

    struct Recorder {
        marker: &'static str,
        log: Arc<Mutex<Vec<&'static str>>>,
    }

    impl GrpcOutboundInterceptor for Recorder {
        fn before_call(&self, _: &mut GrpcRequest) -> Result<(), GrpcOutboundError> {
            self.log.lock().unwrap().push(self.marker);
            Ok(())
        }
        fn after_call(&self, _: &mut GrpcResponse) -> Result<(), GrpcOutboundError> {
            self.log.lock().unwrap().push(self.marker);
            Ok(())
        }
    }

    struct AlwaysFailBefore;

    impl GrpcOutboundInterceptor for AlwaysFailBefore {
        fn before_call(&self, _: &mut GrpcRequest) -> Result<(), GrpcOutboundError> {
            Err(GrpcOutboundError::Status(
                GrpcStatusCode::PermissionDenied,
                "denied by interceptor".into(),
            ))
        }
        fn after_call(&self, _: &mut GrpcResponse) -> Result<(), GrpcOutboundError> {
            Ok(())
        }
    }

    struct CountAfter(Arc<AtomicUsize>);

    impl GrpcOutboundInterceptor for CountAfter {
        fn before_call(&self, _: &mut GrpcRequest) -> Result<(), GrpcOutboundError> {
            Ok(())
        }
        fn after_call(&self, _: &mut GrpcResponse) -> Result<(), GrpcOutboundError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    fn req() -> GrpcRequest {
        GrpcRequest::new("svc/M", vec![], Duration::from_secs(1))
    }
    fn resp() -> GrpcResponse {
        GrpcResponse {
            body: vec![],
            metadata: GrpcMetadata::default(),
        }
    }

    /// @covers: is_empty
    #[test]
    fn test_new_chain_is_empty() {
        let chain = GrpcOutboundInterceptorChain::new();
        assert_eq!(chain.len(), 0);
        assert!(chain.is_empty());
    }

    /// @covers: push
    #[test]
    fn test_push_appends_in_registration_order() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let chain = GrpcOutboundInterceptorChain::new()
            .push(Arc::new(Recorder {
                marker: "a",
                log: log.clone(),
            }))
            .push(Arc::new(Recorder {
                marker: "b",
                log: log.clone(),
            }))
            .push(Arc::new(Recorder {
                marker: "c",
                log: log.clone(),
            }));
        let mut r = req();
        chain.run_before(&mut r).expect("chain should pass");
        assert_eq!(log.lock().unwrap().clone(), vec!["a", "b", "c"]);
    }

    /// @covers: run_before
    #[test]
    fn test_run_before_short_circuits_on_first_failure() {
        let after_count = Arc::new(AtomicUsize::new(0));
        let chain = GrpcOutboundInterceptorChain::new()
            .push(Arc::new(AlwaysFailBefore))
            .push(Arc::new(CountAfter(after_count.clone())));
        let mut r = req();
        match chain.run_before(&mut r) {
            Err(GrpcOutboundError::Status(code, _)) => {
                assert_eq!(code, GrpcStatusCode::PermissionDenied);
            }
            other => panic!("expected PermissionDenied, got {other:?}"),
        }
        assert_eq!(after_count.load(Ordering::SeqCst), 0);
    }

    /// @covers: run_after
    #[test]
    fn test_run_after_invokes_every_interceptor_in_order() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let chain = GrpcOutboundInterceptorChain::new()
            .push(Arc::new(Recorder {
                marker: "x",
                log: log.clone(),
            }))
            .push(Arc::new(Recorder {
                marker: "y",
                log: log.clone(),
            }));
        let mut r = resp();
        chain.run_after(&mut r).expect("chain should pass");
        assert_eq!(log.lock().unwrap().clone(), vec!["x", "y"]);
    }

    /// @covers: len
    #[test]
    fn test_len_returns_number_of_registered_interceptors() {
        let chain = GrpcOutboundInterceptorChain::new().push(Arc::new(AlwaysFailBefore));
        assert_eq!(chain.len(), 1);
        assert!(!chain.is_empty());
    }
}
