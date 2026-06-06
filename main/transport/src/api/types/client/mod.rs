//! Client interface counterpart.
//!
//! The neutral `GrpcClientBuilder` marker lives here. The concrete
//! hyper/tonic-backed client and its builders live in `spi/client/tonic/`.

pub mod grpc_client_builder;
