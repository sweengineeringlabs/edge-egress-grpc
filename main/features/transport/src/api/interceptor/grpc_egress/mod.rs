//! gRPC outbound interceptor types.

pub mod grpc;

pub use grpc::GrpcEgressInterceptor;
pub use grpc::GrpcEgressInterceptorChain;
