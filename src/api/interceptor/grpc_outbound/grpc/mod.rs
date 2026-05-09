//! gRPC outbound interceptor types (prefix-grouped under grpc/).

pub mod grpc_outbound_interceptor;
pub mod grpc_outbound_interceptor_chain;

pub use grpc_outbound_interceptor::GrpcOutboundInterceptor;
pub use grpc_outbound_interceptor_chain::GrpcOutboundInterceptorChain;
