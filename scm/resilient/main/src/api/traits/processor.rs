//! `Processor` trait — primary processing contract for this crate.

use futures::future::BoxFuture;

use crate::api::error::resilient_transport_error::ResilientTransportError;

/// Primary processing trait — required because `service_type = "processor"` in Cargo.toml.
///
/// Implemented by `GrpcResilientSvc` in `core/`.
pub trait Processor: Send + Sync {
    /// Assemble and return a resilient gRPC transport.
    ///
    /// Returns `Err` when the channel or resilience configuration is invalid.
    fn process(
        &self,
        config: &swe_edge_egress_grpc::GrpcChannelConfig,
    ) -> BoxFuture<
        '_,
        Result<std::sync::Arc<dyn swe_edge_egress_grpc::GrpcEgress>, ResilientTransportError>,
    >;

    /// Identify this processor unit for logging and metrics.
    fn describe(&self) -> &'static str;
}
