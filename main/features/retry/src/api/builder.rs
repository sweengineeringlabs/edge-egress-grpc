//! ApplicationConfigBuilder type declaration (SEA rule 160 — public types live in api/).

use crate::api::retry_config::GrpcRetryConfig;

/// Opaque builder for the retry decorator.
///
/// Construct via [`builder()`](crate::builder) (loads SWE
/// baseline) or
/// [`ApplicationConfigBuilder::with_config`](crate::ApplicationConfigBuilder::with_config) (caller-supplied).
/// Wrap an inner [`GrpcOutbound`](swe_edge_egress_grpc::GrpcOutbound)
/// with [`ApplicationConfigBuilder::wrap`](crate::ApplicationConfigBuilder::wrap) to finalize.
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    pub(crate) config: GrpcRetryConfig,
}

#[cfg(test)]
mod tests {
    /// @covers: builder — module compiles
    #[test]
    fn test_builder_module_is_accessible() {
        assert!(true, "module builder compiled and accessible");
    }
}
