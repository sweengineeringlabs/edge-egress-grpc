//! `impl Processor for GrpcBreakerSvc` — satisfies `service_type = "processor"`.

use crate::api::traits::Processor;
use crate::api::types::grpc_breaker_svc::GrpcBreakerSvc;

impl Processor for GrpcBreakerSvc {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "grpc-breaker";
        LABEL
    }
}
