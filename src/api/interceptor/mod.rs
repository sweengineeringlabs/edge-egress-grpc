//! gRPC outbound interceptor traits and built-in types.

pub mod grpc_outbound;
pub mod trace_context;

pub use grpc_outbound::GrpcOutboundInterceptor;
pub use grpc_outbound::GrpcOutboundInterceptorChain;
pub use trace_context::TraceContextInterceptor;
pub use trace_context::TraceContextSource;
