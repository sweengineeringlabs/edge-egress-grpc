//! `TransportSvc` — impl blocks for the transport SAF facade.

use std::sync::Arc;
use std::time::Duration;

use crate::api::{
    ApplicationConfigBuilder, GrpcChannelConfig, GrpcChannelConfigError, GrpcEgress,
    GrpcEgressError, Processor, ResilienceConfig, TransportSvc, ValidationRequest, Validator,
    DEFAULT_REQUEST_TIMEOUT_SECS,
};
use crate::spi::client::tonic::{TonicGrpcClient, TonicGrpcClientProtocol};
use crate::spi::loadbalancer::tonic::TonicLbGrpcClient;
use swe_edge_loadbalancer::LoadbalancerConfig;

impl TransportSvc {
    /// Create a config builder pre-populated with this crate's name and version.
    pub fn create_config_builder() -> ApplicationConfigBuilder {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        ApplicationConfigBuilder(b)
    }

    /// Construct a boxed [`GrpcEgress`] from a validated [`GrpcChannelConfig`].
    pub fn create_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
        let client = Self::create_tonic_client_from_config(config)?;
        let transport: Arc<dyn GrpcEgress> = Arc::new(client);
        Ok(transport)
    }

    /// Construct a concrete [`GrpcEgress`] + [`Processor`] transport from a
    /// validated [`GrpcChannelConfig`].
    ///
    /// Returns an opaque `impl GrpcEgress + Processor` rather than naming the
    /// concrete adapter type directly (see SEA rule `pub_types_in_api_only`);
    /// callers that need to compose further (e.g. wrapping in a retry/breaker
    /// decorator) can still do so generically.
    pub fn create_tonic_client_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<impl GrpcEgress + Processor, GrpcChannelConfigError> {
        if config.tls_required && TonicGrpcClientProtocol::is_plaintext_endpoint(&config.endpoint) {
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
    pub fn validate_resilience_config(
        config: &ResilienceConfig,
    ) -> Result<(), GrpcChannelConfigError> {
        config.validate(ValidationRequest)
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
