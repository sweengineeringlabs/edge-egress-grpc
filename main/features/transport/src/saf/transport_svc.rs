//! `TransportSvc` — impl blocks for the transport SAF facade.

use std::sync::Arc;
use std::time::Duration;

use crate::api::port::{GrpcChannelConfigError, GrpcEgress};
use crate::api::traits::Validator;
use crate::api::types::{TonicGrpcClient, TransportSvc};
use crate::api::value::{GrpcChannelConfig, ResilienceConfig, DEFAULT_REQUEST_TIMEOUT_SECS};
use crate::core::client::tonic_grpc_client::is_plaintext_endpoint;

impl TransportSvc {
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    pub fn create_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
        let client = Self::create_tonic_client_from_config(config)?;
        let transport: Arc<dyn GrpcEgress> = Arc::new(client);
        Ok(transport)
    }

    pub fn create_tonic_client_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<TonicGrpcClient, GrpcChannelConfigError> {
        if config.tls_required && is_plaintext_endpoint(&config.endpoint) {
            return Err(GrpcChannelConfigError::PlaintextRejected(
                config.endpoint.clone(),
            ));
        }
        let timeout = Duration::from_secs(
            config
                .request_timeout_secs
                .unwrap_or(DEFAULT_REQUEST_TIMEOUT_SECS),
        );
        let mut client = TonicGrpcClient::with_timeout(&config.endpoint, timeout);
        client.max_message_bytes = config.max_message_bytes;
        client.compression = config.compression;
        Ok(client)
    }

    pub fn validate_resilience_config(config: &ResilienceConfig) -> Result<(), String> {
        let result = config.validate();
        result
    }
}
