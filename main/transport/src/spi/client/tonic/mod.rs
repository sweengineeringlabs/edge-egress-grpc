//! Hyper/hyper-rustls-backed gRPC client (`TonicGrpcClient`) and its builders.
//!
//! These are external-library wrapper implementations of the `api/` `GrpcEgress`
//! and `Processor` contracts; they are exported solely through `saf/`.
pub(crate) mod tonic_grpc_client;
pub(crate) mod tonic_grpc_client_builder;
pub(crate) mod tonic_grpc_client_core_builder;
pub(crate) mod tonic_grpc_client_impl;

pub use tonic_grpc_client::TonicGrpcClient;
pub use tonic_grpc_client_builder::TonicGrpcClientBuilder;
pub(crate) use tonic_grpc_client_impl::TonicGrpcClientCore;
