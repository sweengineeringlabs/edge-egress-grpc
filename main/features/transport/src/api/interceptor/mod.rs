//! gRPC outbound interceptor traits and built-in types.

pub mod grpc;
pub mod trace;

pub use grpc::GrpcEgressInterceptor;
pub use grpc::GrpcEgressInterceptorChain;
pub use trace::TraceContextInterceptor;
pub use trace::TraceContextSource;
