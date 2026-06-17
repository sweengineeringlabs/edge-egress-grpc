//! `TransportSvc` — impl blocks for the transport SAF facade.

use std::sync::Arc;
use std::time::Duration;

use swe_edge_loadbalancer::LoadbalancerConfig;

use crate::api::error::{GrpcChannelConfigError, GrpcEgressError};
use crate::api::traits::GrpcEgress;
use crate::api::traits::Validator;
use crate::api::types::TransportSvc;
use crate::api::types::{GrpcChannelConfig, ResilienceConfig, DEFAULT_REQUEST_TIMEOUT_SECS};
use crate::spi::client::tonic::{TonicGrpcClient, TonicGrpcClientCore};
use crate::spi::loadbalancer::tonic::TonicLbGrpcClient;

impl TransportSvc {
    /// Create a config builder pre-populated with this crate's name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Construct a boxed [`GrpcEgress`] from a validated [`GrpcChannelConfig`].
    pub fn create_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
        let client = Self::create_tonic_client_from_config(config)?;
        let transport: Arc<dyn GrpcEgress> = Arc::new(client);
        Ok(transport)
    }

    /// Construct a concrete [`TonicGrpcClient`] from a validated [`GrpcChannelConfig`].
    pub fn create_tonic_client_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<TonicGrpcClient, GrpcChannelConfigError> {
        if config.tls_required && TonicGrpcClientCore::is_plaintext_endpoint(&config.endpoint) {
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

    /// Validate a [`ResilienceConfig`], returning the first constraint violation as `Err`.
    pub fn validate_resilience_config(config: &ResilienceConfig) -> Result<(), String> {
        config.validate()
    }

    /// Construct a load-balanced [`GrpcEgress`] from a [`LoadbalancerConfig`].
    ///
    /// Uses `tonic::transport::Channel::balance_list` for transport-level routing
    /// and [`swe_edge_loadbalancer`] for health-aware backend selection.
    ///
    /// # Errors
    ///
    /// Returns [`GrpcEgressError::Unavailable`] when the config has no backends
    /// or any backend URL is not a valid URI.
    pub fn create_lb_transport_from_config(
        config: LoadbalancerConfig,
    ) -> Result<Arc<dyn GrpcEgress>, GrpcEgressError> {
        let client = TonicLbGrpcClient::from_config(config)?;
        Ok(Arc::new(client))
    }
}
