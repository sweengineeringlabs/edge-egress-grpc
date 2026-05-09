//! Interface counterpart for `core/resilience/retry/retry_policy.rs`.
//!
//! The `RetryPolicyPort` trait is the public contract for retry policy implementations.

pub use crate::api::resilience::retry::retry_policy_port::RetryPolicyPort;
pub use crate::api::resilience::retry::retry_outcome::RetryOutcome;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_port_re_export_is_object_safe() {
        fn _assert(_: &dyn RetryPolicyPort) {}
    }
}
