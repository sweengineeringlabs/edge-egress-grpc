//! `RetryPolicyBuilder` — interface counterpart for `core/resilience/retry/retry_policy_builder.rs`.

/// Builder marker type for retry policies.
///
/// The concrete builder lives in `core/`; the public interface is
/// [`crate::api::resilience::retry::retry_policy_port::RetryPolicyPort`].
pub struct RetryPolicyBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_builder_is_constructable() {
        let _ = RetryPolicyBuilder;
    }
}
