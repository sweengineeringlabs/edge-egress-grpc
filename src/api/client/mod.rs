//! Client interface counterpart for `core/client/`.

pub mod grpc_client_builder;
pub mod tonic_grpc_client;

pub use crate::api::port::GrpcOutbound;
pub use crate::api::port::GrpcOutboundError;
pub use crate::api::port::GrpcOutboundResult;
