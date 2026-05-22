//! gRPC outbound interceptor types (prefix-grouped under grpc/).

pub mod grpc_egress_interceptor;
pub mod grpc_egress_interceptor_chain;

pub use grpc_egress_interceptor::GrpcEgressInterceptor;
pub use grpc_egress_interceptor_chain::GrpcEgressInterceptorChain;
