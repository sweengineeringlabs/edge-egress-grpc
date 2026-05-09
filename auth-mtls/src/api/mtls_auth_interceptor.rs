//! Struct declaration and constructors for [`MtlsAuthInterceptor`].

use crate::api::MtlsAuthConfig;

/// `GrpcInboundInterceptor` that enforces mTLS-derived identity.
#[derive(Debug, Clone)]
pub struct MtlsAuthInterceptor {
    pub(crate) config: MtlsAuthConfig,
}

impl MtlsAuthInterceptor {
    /// Construct from config.
    pub fn from_config(config: MtlsAuthConfig) -> Self { Self { config } }
    /// Convenience: accept any peer that completed mTLS, no allowlist.
    pub fn allow_any_verified_peer() -> Self {
        Self::from_config(MtlsAuthConfig::allow_any_verified_peer())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_config
    #[test]
    fn test_from_config_creates_interceptor() {
        let _ = MtlsAuthInterceptor::from_config(MtlsAuthConfig::allow_any_verified_peer());
    }

    /// @covers: allow_any_verified_peer
    #[test]
    fn test_allow_any_verified_peer_creates_unrestricted_interceptor() {
        let _ = MtlsAuthInterceptor::allow_any_verified_peer();
    }
}
