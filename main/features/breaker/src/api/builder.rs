//! ApplicationConfigBuilder type declaration (SEA rule 160 — public types live in api/).

use crate::api::breaker_config::GrpcBreakerConfig;

/// Opaque builder for the breaker decorator.
///
/// Construct via [`builder()`](crate::builder) (loads SWE
/// baseline) or
/// [`ApplicationConfigBuilder::with_config`](crate::ApplicationConfigBuilder::with_config) (caller-supplied).
/// Wrap an inner [`GrpcOutbound`](swe_edge_egress_grpc::GrpcOutbound)
/// with [`ApplicationConfigBuilder::wrap`](crate::ApplicationConfigBuilder::wrap) to finalize.
#[derive(Debug)]
pub struct ApplicationConfigBuilder {
    pub(crate) config: GrpcBreakerConfig,
}

#[cfg(test)]
mod tests {
    /// @covers: builder — module compiles
    #[test]
    fn test_builder_module_is_accessible() {}
}
