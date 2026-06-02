//! Error types for the gRPC retry decorator.

#[allow(clippy::module_inception)]
pub mod error;
pub use error::Error;
pub mod retry_domain_error;
