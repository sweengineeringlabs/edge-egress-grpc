//! ApplicationConfigBuilder type declaration (SEA rule 160 — public types live in api/).

use crate::api::retry_config::GrpcRetryConfig;

/// Opaque builder for the retry decorator.
///
/// Construct via [`builder()`](crate::builder) (loads SWE
/// baseline) or
/// [`ApplicationConfigBuilder::with_config`](crate::ApplicationConfigBuilder::with_config) (caller-supplied).
/// Wrap an inner [`GrpcEgress`](swe_edge_egress_grpc::GrpcEgress)
/// with [`ApplicationConfigBuilder::wrap`](crate::ApplicationConfigBuilder::wrap) to finalize.
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    pub(crate) config: GrpcRetryConfig,
}

#[cfg(test)]
mod tests {
    use super::ApplicationConfigBuilder;
    use crate::api::retry_config::GrpcRetryConfig;

    #[test]
    fn test_application_config_builder_stores_config_max_attempts() {
        let cfg = GrpcRetryConfig::from_config(
            r#"
                max_attempts = 7
                initial_backoff_ms = 50
                backoff_multiplier = 1.5
                jitter_factor = 0.0
                max_backoff_ms = 500
                rate_limit_max_attempts = 2
                rate_limit_initial_backoff_ms = 100
                rate_limit_max_backoff_ms = 1000
            "#,
        )
        .unwrap();
        let b = ApplicationConfigBuilder { config: cfg };
        assert_eq!(b.config.max_attempts, 7);
        assert_eq!(b.config.initial_backoff_ms, 50);
    }
}
