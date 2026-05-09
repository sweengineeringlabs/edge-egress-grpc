//! `RetryPolicyPort` trait — interface contract for retry policy implementations.

use crate::api::port::GrpcOutboundError;
use crate::api::resilience::retry::retry_outcome::RetryOutcome;

/// Interface contract for a retry policy.
///
/// Implemented by `crate::core::resilience::retry::retry_policy::RetryPolicy`.
pub trait RetryPolicyPort {
    /// Decide whether to retry an error on a given attempt index (0-based).
    fn decide(&self, err: &GrpcOutboundError, retry_index: u32) -> RetryOutcome;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_port_is_object_safe() {
        fn _assert(_: &dyn RetryPolicyPort) {}
    }
}
