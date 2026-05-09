//! `GrpcOutboundInterceptor` trait — observes/mutates requests before and after dispatch.

use crate::api::port::GrpcOutboundError;
use crate::api::value_object::{GrpcRequest, GrpcResponse};

/// An interceptor for outbound gRPC calls.
pub trait GrpcOutboundInterceptor: Send + Sync {
    /// Run before the request is sent on the wire.
    /// Returning `Err(_)` aborts the call — the transport is not invoked.
    fn before_call(&self, req: &mut GrpcRequest) -> Result<(), GrpcOutboundError>;

    /// Run after a successful response has been read from the wire.
    /// Returning `Err(_)` converts the call result to that error.
    fn after_call(&self, resp: &mut GrpcResponse) -> Result<(), GrpcOutboundError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_outbound_interceptor_is_object_safe() {
        fn _assert(_: &dyn GrpcOutboundInterceptor) {}
    }
}
