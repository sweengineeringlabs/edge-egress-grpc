//! Struct declaration and constructors for [`BearerInboundInterceptor`].

use crate::api::BearerInboundConfig;

/// `GrpcInboundInterceptor` that validates incoming JWT bearer tokens.
pub struct BearerInboundInterceptor {
    pub(crate) config: BearerInboundConfig,
}

impl BearerInboundInterceptor {
    /// Construct from config.
    pub fn from_config(config: BearerInboundConfig) -> Self { Self { config } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{BearerInboundConfig, BearerSecret};

    fn test_cfg() -> BearerInboundConfig {
        BearerInboundConfig {
            secret: BearerSecret::Hs256 { secret: b"key".to_vec() },
            expected_issuer: "iss".into(),
            expected_audience: "aud".into(),
            leeway_seconds: 0,
        }
    }

    /// @covers: from_config
    #[test]
    fn test_from_config_creates_interceptor() {
        let _ = BearerInboundInterceptor::from_config(test_cfg());
    }
}
