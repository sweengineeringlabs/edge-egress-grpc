//! ApplicationConfigBuilder type declaration (SEA rule 160 — public types live in api/).

use crate::api::breaker_config::GrpcBreakerConfig;

/// Opaque builder for the breaker decorator.
///
/// Construct via [`builder()`](crate::builder) (loads SWE
/// baseline) or
/// [`ApplicationConfigBuilder::with_config`](crate::ApplicationConfigBuilder::with_config) (caller-supplied).
/// Wrap an inner [`GrpcEgress`](swe_edge_egress_grpc::GrpcEgress)
/// with [`ApplicationConfigBuilder::wrap`](crate::ApplicationConfigBuilder::wrap) to finalize.
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    pub(crate) config: GrpcBreakerConfig,
}

#[cfg(test)]
mod tests {
    use super::ApplicationConfigBuilder;
    use crate::api::breaker_config::GrpcBreakerConfig;

    #[test]
    fn test_application_config_builder_stores_config_failure_threshold() {
        let cfg = GrpcBreakerConfig::from_config(
            "failure_threshold = 9\ncool_down_seconds = 45\nhalf_open_probe_count = 2",
        )
        .unwrap();
        let b = ApplicationConfigBuilder { config: cfg };
        assert_eq!(b.config.failure_threshold, 9);
        assert_eq!(b.config.cool_down_seconds, 45);
    }
}
