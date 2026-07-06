//! `impl Processor for GrpcResilientSvc` — satisfies `service_type = "processor"`.

use crate::api::ResilientTransportError;
use crate::api::{DescribeRequest, DescribeResponse, GrpcResilientSvc, Processor};

impl Processor for GrpcResilientSvc {
    fn describe(&self, _req: DescribeRequest) -> Result<DescribeResponse, ResilientTransportError> {
        Ok(DescribeResponse {
            label: "grpc-resilient",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn test_describe_returns_grpc_resilient_label() {
        let resp = GrpcResilientSvc
            .describe(DescribeRequest)
            .expect("infallible");
        assert_eq!(resp.label, "grpc-resilient");
    }
}
