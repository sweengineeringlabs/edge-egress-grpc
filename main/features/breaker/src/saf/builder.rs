//! Public factory + wrapper entry points.

use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_egress_grpc::GrpcEgress;

use crate::api::breaker_client::GrpcBreakerClient;
use crate::api::breaker_config::GrpcBreakerConfig;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Wrap `inner` with the supplied breaker policy.
///
/// Use this when you have already loaded `config` via
/// [`GrpcBreakerConfig::load`] or [`GrpcBreakerConfig::from_config`].
/// For one-shot default wrapping use [`create_breaker_client`].
pub fn wrap_breaker<T: GrpcEgress + Send + Sync + 'static>(
    inner: T,
    config: GrpcBreakerConfig,
) -> GrpcBreakerClient<T> {
    GrpcBreakerClient::new(inner, config)
}

/// One-shot factory: wrap `inner` with the SWE default breaker policy.
///
/// Equivalent to `wrap_breaker(inner, GrpcBreakerConfig::default())`.
/// Use [`wrap_breaker`] when you need to override the policy before wrapping.
pub fn create_breaker_client<T: GrpcEgress + Send + Sync + 'static>(
    inner: T,
) -> GrpcBreakerClient<T> {
    GrpcBreakerClient::new(inner, GrpcBreakerConfig::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: GrpcBreakerConfig::section_name
    #[test]
    fn test_grpc_breaker_config_section_name_is_grpc_breaker() {
        use swe_edge_configbuilder::ConfigSection as _;
        assert_eq!(GrpcBreakerConfig::section_name(), "grpc_breaker");
    }

    /// @covers: GrpcBreakerConfig::default
    #[test]
    fn test_grpc_breaker_config_default_has_positive_failure_threshold() {
        assert!(GrpcBreakerConfig::default().failure_threshold >= 1);
    }
}
