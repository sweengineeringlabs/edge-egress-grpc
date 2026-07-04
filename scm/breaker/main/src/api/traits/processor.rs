//! `Processor` trait — primary processing contract for the breaker crate.

use crate::api::error::breaker_domain_error::BreakerDomainError;
use crate::api::types::describe_request::DescribeRequest;
use crate::api::types::describe_response::DescribeResponse;

/// Primary processing trait for this crate (service_type = "processor").
/// Every circuit-breaker middleware produced by this crate implements it.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self, req: DescribeRequest) -> Result<DescribeResponse, BreakerDomainError>;
}
