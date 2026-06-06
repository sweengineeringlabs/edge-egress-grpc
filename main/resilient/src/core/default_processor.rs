//! `impl Processor for GrpcResilientSvc` — satisfies `service_type = "processor"`.

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcEgress};

use crate::api::error::resilient_transport_error::ResilientTransportError;
use crate::api::traits::processor::Processor;
use crate::api::types::grpc_resilient_svc::GrpcResilientSvc;

impl Processor for GrpcResilientSvc {
    fn process(
        &self,
        config: &GrpcChannelConfig,
    ) -> BoxFuture<'_, Result<Arc<dyn GrpcEgress>, ResilientTransportError>> {
        let result = GrpcResilientSvc::create_resilient_transport_from_config(config);
        Box::pin(std::future::ready(result))
    }

    fn describe(&self) -> &'static str {
        const LABEL: &str = "grpc-resilient";
        LABEL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn test_describe_returns_grpc_resilient_label() {
        let svc = GrpcResilientSvc;
        assert_eq!(svc.describe(), "grpc-resilient");
    }
}
