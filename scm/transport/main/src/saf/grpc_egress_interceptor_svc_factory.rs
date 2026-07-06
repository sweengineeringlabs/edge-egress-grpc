//! Composition site for [`GrpcEgressInterceptor`] — one file per trait keeps wiring focused.

use crate::api::{GrpcEgressInterceptor, TraceContextInterceptor};

/// Factory for the default [`GrpcEgressInterceptor`].
pub struct GrpcEgressInterceptorFactory;

impl GrpcEgressInterceptorFactory {
    /// Construct a pass-through [`TraceContextInterceptor`] as the default interceptor.
    pub fn create() -> Box<dyn GrpcEgressInterceptor> {
        Box::new(TraceContextInterceptor::pass_through())
    }
}
