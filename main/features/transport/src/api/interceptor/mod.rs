//! gRPC outbound interceptor traits and built-in types.

pub mod grpc_egress;
pub mod trace_context;

pub use grpc_egress::GrpcEgressInterceptor;
pub use grpc_egress::GrpcEgressInterceptorChain;
pub use trace_context::TraceContextInterceptor;
pub use trace_context::TraceContextSource;
