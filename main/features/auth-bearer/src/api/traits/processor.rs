//! Primary processing contract for bearer egress interceptors.

/// Primary processing contract for bearer egress interceptors.
///
/// Implemented by [`crate::BearerEgressInterceptor`] to mark the type as a
/// first-class processor in the SEA pipeline.
pub trait Processor: Send + Sync {}
