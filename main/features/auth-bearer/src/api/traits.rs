//! SEA interface contracts.
//!
//! `Processor` is defined in [`crate::api::processor`] and re-exported
//! here.  `Validator` is defined inline — rule 155 requires the literal
//! `pub trait Validator` to appear in this file.

pub use crate::api::processor::Processor;

/// Validation contract — implementors check their own invariants.
///
/// Implemented by [`crate::api::bearer::bearer_egress_config::BearerEgressConfig`]
/// to verify that required fields (issuer, audience, subject, secret) are
/// populated before the interceptor is used.
#[allow(dead_code)]
pub trait Validator: Send + Sync {
    /// Validate the receiver's invariants.
    ///
    /// Returns `Ok(())` when all invariants hold, or an `Err` describing the
    /// first violation found.
    fn validate(&self) -> Result<(), String>;
}
