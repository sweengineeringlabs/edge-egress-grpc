//! `Processor` trait — primary processing contract for the breaker crate.

/// Primary processing trait for this crate (service_type = "processor").
/// Every circuit-breaker middleware produced by this crate implements it.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
