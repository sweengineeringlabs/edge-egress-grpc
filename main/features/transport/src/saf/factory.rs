//! Factory functions — build outbound transports from config.

use std::sync::Arc;

use crate::api::port::{GrpcChannelConfigError, GrpcEgress};
use crate::api::types::TonicGrpcClient;

/// Return a config builder pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

/// Build an outbound transport from a [`crate::api::value_object::GrpcChannelConfig`].
///
/// Returns [`GrpcChannelConfigError::PlaintextRejected`] when
/// `config.tls_required = true` and the endpoint scheme is `http://`.
pub fn create_transport_from_config(
    config: &crate::api::value_object::GrpcChannelConfig,
) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
    Ok(Arc::new(TonicGrpcClient::from_config(config)?))
}

/// Build a concrete [`TonicGrpcClient`] from a [`crate::api::value_object::GrpcChannelConfig`].
///
/// Unlike [`create_transport_from_config`], returns the concrete type so that
/// callers (e.g. `swe-edge-egress-grpc-resilient`) can wrap it in decorator
/// layers before erasing the type to `Arc<dyn GrpcEgress>`.
pub fn create_tonic_client_from_config(
    config: &crate::api::value_object::GrpcChannelConfig,
) -> Result<TonicGrpcClient, GrpcChannelConfigError> {
    TonicGrpcClient::from_config(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::value_object::GrpcChannelConfig;

    /// @covers: create_transport_from_config
    #[test]
    fn test_create_transport_from_config_without_resilience_returns_ok() {
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
        assert!(create_transport_from_config(&config).is_ok());
    }

    /// @covers: create_transport_from_config
    #[test]
    fn test_create_transport_from_config_tls_required_rejects_plaintext() {
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
        assert!(matches!(
            create_transport_from_config(&config),
            Err(GrpcChannelConfigError::PlaintextRejected(_))
        ));
    }

    /// @covers: create_tonic_client_from_config
    #[test]
    fn test_create_tonic_client_from_config_plaintext_allowed_returns_ok() {
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051").allow_plaintext();
        assert!(create_tonic_client_from_config(&config).is_ok());
    }

    /// @covers: create_tonic_client_from_config
    #[test]
    fn test_create_tonic_client_from_config_tls_required_rejects_plaintext_endpoint() {
        let config = GrpcChannelConfig::new("http://127.0.0.1:50051");
        assert!(matches!(
            create_tonic_client_from_config(&config),
            Err(GrpcChannelConfigError::PlaintextRejected(_))
        ));
    }
}
