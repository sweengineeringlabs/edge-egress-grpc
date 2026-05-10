//! Struct declaration and constructors for [`BearerOutboundInterceptor`].

use crate::api::BearerOutboundConfig;

/// `GrpcOutboundInterceptor` that signs and attaches a JWT bearer token.
pub struct BearerOutboundInterceptor {
    pub(crate) config: BearerOutboundConfig,
}

impl BearerOutboundInterceptor {
    /// Construct from config.
    pub fn from_config(config: BearerOutboundConfig) -> Self { Self { config } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{BearerOutboundConfig, BearerSecret};

    fn test_cfg() -> BearerOutboundConfig {
        BearerOutboundConfig {
            secret: BearerSecret::Hs256 { secret: b"key".to_vec() },
            issuer: "iss".into(),
            audience: "aud".into(),
            subject: "sub".into(),
            lifetime_seconds: 300,
        }
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_creates_outbound_interceptor() {
        let _ = BearerOutboundInterceptor::from_config(test_cfg());
    }
}
