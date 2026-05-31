//! `Processor` trait — primary processing contract for this crate.

use futures::future::BoxFuture;

use crate::api::port::GrpcEgressError;

/// Primary processing trait — required because `service_type = "processor"` in Cargo.toml.
///
/// Implemented by `TonicGrpcClient` and `ResilientGrpcClient`.
pub trait Processor: Send + Sync {
    /// Execute this processor unit's primary operation.
    ///
    /// Returns `Err` when the underlying transport or business logic fails.
    fn process(&self) -> BoxFuture<'_, Result<(), GrpcEgressError>>;

    /// Identify this processor unit for logging and metrics.
    fn describe(&self) -> &'static str;
}
