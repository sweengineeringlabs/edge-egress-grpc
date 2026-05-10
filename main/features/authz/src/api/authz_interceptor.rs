//! Struct declaration and constructors for [`AuthzInterceptor`].

use std::sync::Arc;

use crate::api::AuthzPolicy;

/// Inbound interceptor that runs an [`AuthzPolicy`] against the caller's identity.
pub struct AuthzInterceptor {
    pub(crate) policy: Arc<dyn AuthzPolicy>,
}

impl AuthzInterceptor {
    /// Construct from any policy.
    pub fn from_policy<P: AuthzPolicy + 'static>(policy: P) -> Self {
        Self {
            policy: Arc::new(policy),
        }
    }
    /// Construct from an already-shared policy.
    pub fn from_shared_policy(policy: Arc<dyn AuthzPolicy>) -> Self {
        Self { policy }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AllowAll;
    impl AuthzPolicy for AllowAll {
        fn allows(&self, _: &swe_edge_ingress_grpc::PeerIdentity, _: &str) -> bool {
            true
        }
    }

    /// @covers: from_policy
    #[test]
    fn test_from_policy_creates_interceptor() {
        let _ = AuthzInterceptor::from_policy(AllowAll);
    }

    /// @covers: from_shared_policy
    #[test]
    fn test_from_shared_policy_creates_interceptor() {
        let _ = AuthzInterceptor::from_shared_policy(Arc::new(AllowAll));
    }
}
