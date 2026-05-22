//! Built-in outbound interceptor implementations.
//!
//! `TraceContextInterceptor` is implemented directly in the api/ layer since
//! its `GrpcEgressInterceptor` impl uses only api/-scoped types.
