//! Struct declaration and constructors for [`BearerEgressInterceptor`].

use crate::api::BearerEgressConfig;

/// `GrpcEgressInterceptor` that signs and attaches a JWT bearer token.
pub struct BearerEgressInterceptor {
    pub(crate) config: BearerEgressConfig,
}

impl BearerEgressInterceptor {
    /// Construct from config.
    pub fn from_config(config: BearerEgressConfig) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{BearerEgressConfig, BearerSecret};

    fn test_cfg() -> BearerEgressConfig {
        BearerEgressConfig {
            secret: BearerSecret::Hs256 {
                secret: b"key".to_vec(),
            },
            issuer: "iss".into(),
            audience: "aud".into(),
            subject: "sub".into(),
            lifetime_seconds: 300,
        }
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_creates_outbound_interceptor() {
        let _ = BearerEgressInterceptor::from_config(test_cfg());
    }
}
