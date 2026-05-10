//! Factory functions — build outbound transports from config.

use std::sync::Arc;

use crate::api::client::tonic_grpc_client::TonicGrpcClient;
use crate::api::port::{GrpcChannelConfigError, GrpcOutbound};

/// Build an outbound transport from a [`crate::api::value_object::GrpcChannelConfig`].
///
/// Returns [`GrpcChannelConfigError::PlaintextRejected`] when
/// `config.tls_required = true` and the endpoint scheme is `http://`.
pub fn create_transport_from_config(
    config: &crate::api::value_object::GrpcChannelConfig,
) -> Result<Arc<dyn GrpcOutbound>, GrpcChannelConfigError> {
    Ok(Arc::new(TonicGrpcClient::from_config(config)?))
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
}
