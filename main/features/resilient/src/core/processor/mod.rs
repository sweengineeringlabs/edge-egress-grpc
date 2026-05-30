//! `impl Processor for GrpcResilientSvc` — satisfies `service_type = "processor"`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress};

use crate::api::error::resilient_transport_error::ResilientTransportError;
use crate::api::traits::Processor;
use crate::api::types::grpc_resilient_svc::GrpcResilientSvc;
use crate::core::factory::ResilientAssembler;

impl Processor for GrpcResilientSvc {
    fn process(
        &self,
        config: &GrpcChannelConfig,
    ) -> BoxFuture<'_, Result<Arc<dyn GrpcEgress>, ResilientTransportError>> {
        let result = ResilientAssembler::assemble(config);
        Box::pin(std::future::ready(result))
    }

    fn describe(&self) -> &'static str {
        "grpc-resilient"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: GrpcResilientSvc::describe
    #[test]
    fn test_describe_returns_grpc_resilient_label() {
        let svc = GrpcResilientSvc;
        assert_eq!(svc.describe(), "grpc-resilient");
    }
}
