//! Error types for the gRPC retry decorator.

pub mod error;
pub use error::Error;
pub mod retry_domain_error;
pub use retry_domain_error::RetryDomainError;
