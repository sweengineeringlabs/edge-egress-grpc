//! Validation contract for bearer auth types.

/// Validation contract — implementors check their own invariants.
///
/// Implemented by [`crate::api::bearer::bearer_egress_config::BearerEgressConfig`]
/// to verify that required fields (issuer, audience, subject, secret) are
/// populated before the interceptor is used.
pub trait Validator: Send + Sync {
    /// Validate the receiver's invariants.
    ///
    /// Returns `Ok(())` when all invariants hold, or an `Err` describing the
    /// first violation found.
    fn validate(&self) -> Result<(), String>;
}
