//! Public factory + wrapper entry points.

use swe_edge_configbuilder::ConfigBuilder as _;
use swe_edge_egress_grpc::GrpcEgress;

use crate::api::retry_client::GrpcRetryClient;
use crate::api::retry_config::GrpcRetryConfig;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> impl swe_edge_configbuilder::ConfigBuilder {
    swe_edge_configbuilder::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Wrap `inner` with the supplied retry policy.
///
/// Use this when you have already loaded `config` via
/// [`GrpcRetryConfig::load`] or [`GrpcRetryConfig::from_config`].
/// For one-shot default wrapping use [`create_retry_client`].
pub fn wrap_retry<T: GrpcEgress + Send + Sync + 'static>(
    inner: T,
    config: GrpcRetryConfig,
) -> GrpcRetryClient<T> {
    GrpcRetryClient::new(inner, config)
}

/// One-shot factory: wrap `inner` with the SWE default retry policy.
///
/// Equivalent to `wrap_retry(inner, GrpcRetryConfig::default())`.
/// Use [`wrap_retry`] when you need to override the policy before wrapping.
pub fn create_retry_client<T: GrpcEgress + Send + Sync + 'static>(
    inner: T,
) -> GrpcRetryClient<T> {
    GrpcRetryClient::new(inner, GrpcRetryConfig::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }

    /// @covers: GrpcRetryConfig::section_name
    #[test]
    fn test_grpc_retry_config_section_name_is_grpc_retry() {
        use swe_edge_configbuilder::ConfigSection as _;
        assert_eq!(GrpcRetryConfig::section_name(), "grpc_retry");
    }

    /// @covers: GrpcRetryConfig::default
    #[test]
    fn test_grpc_retry_config_default_has_positive_max_attempts() {
        assert!(GrpcRetryConfig::default().max_attempts >= 1);
    }
}
