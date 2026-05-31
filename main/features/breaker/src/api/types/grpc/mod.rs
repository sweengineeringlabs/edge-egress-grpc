//! gRPC-specific public types for `swe_edge_egress_grpc_breaker`.

pub mod grpc_breaker_client;
pub mod grpc_breaker_svc;

pub use grpc_breaker_client::GrpcBreakerClient;
pub use grpc_breaker_svc::GrpcBreakerSvc;
