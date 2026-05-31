//! `TransportSvc` — impl blocks for the transport SAF facade.

use std::sync::Arc;
use std::time::Duration;

use crate::api::port::{GrpcChannelConfigError, GrpcEgress};
use crate::api::traits::Validator;
use crate::api::types::{TonicGrpcClient, TransportSvc};
use crate::api::value::{GrpcChannelConfig, ResilienceConfig, DEFAULT_REQUEST_TIMEOUT_SECS};
use crate::core::client::tonic_grpc_client::is_plaintext_endpoint;

impl TransportSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigBuilderImpl::for_crate(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
        )
    }

    /// Build an outbound transport from a [`GrpcChannelConfig`].
    pub fn create_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
        Ok(Arc::new(Self::create_tonic_client_from_config(config)?))
    }

    /// Build a concrete [`TonicGrpcClient`] from a [`GrpcChannelConfig`].
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

    /// Describe a processor unit.
    pub fn describe_processor(processor: &dyn crate::api::traits::Processor) -> &'static str {
        processor.describe()
    }

    /// Validate a [`ResilienceConfig`].
    pub fn validate_resilience_config(config: &ResilienceConfig) -> Result<(), String> {
        config.validate()
    }
}

/// Free-function shims for backward compatibility.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    TransportSvc::create_config_builder()
}

pub fn create_transport_from_config(
    config: &GrpcChannelConfig,
) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
    TransportSvc::create_transport_from_config(config)
}

pub fn create_tonic_client_from_config(
    config: &GrpcChannelConfig,
) -> Result<TonicGrpcClient, GrpcChannelConfigError> {
    TransportSvc::create_tonic_client_from_config(config)
}

pub fn describe_processor(processor: &dyn crate::api::traits::Processor) -> &'static str {
    TransportSvc::describe_processor(processor)
}

pub fn validate_resilience_config(config: &ResilienceConfig) -> Result<(), String> {
    TransportSvc::validate_resilience_config(config)
}
