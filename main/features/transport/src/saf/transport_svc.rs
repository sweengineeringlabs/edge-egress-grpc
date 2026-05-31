//! `TransportSvc` — impl blocks for the transport SAF facade.

use std::sync::Arc;

use crate::api::port::{GrpcChannelConfigError, GrpcEgress};
use crate::api::traits::Validator;
use crate::api::types::{TonicGrpcClient, TransportSvc};
use crate::api::value::{GrpcChannelConfig, ResilienceConfig};

impl TransportSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        // @allow: saf_no_wrapper_methods — adds package name and version, not pure delegation
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Build an outbound transport from a [`GrpcChannelConfig`].
    ///
    /// Returns [`GrpcChannelConfigError::PlaintextRejected`] when
    /// `config.tls_required = true` and the endpoint scheme is `http://`.
    pub fn create_transport_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
        let client = TonicGrpcClient::from_config(config)?;
        Ok(Arc::new(client))
    }

    /// Build a concrete [`TonicGrpcClient`] from a [`GrpcChannelConfig`].
    ///
    /// Unlike [`Self::create_transport_from_config`], returns the concrete type so that
    /// callers can wrap it in decorator layers before erasing the type to `Arc<dyn GrpcEgress>`.
    pub fn create_tonic_client_from_config(
        config: &GrpcChannelConfig,
    ) -> Result<TonicGrpcClient, GrpcChannelConfigError> {
        TonicGrpcClient::from_config(config)
    }

    /// Describe a processor unit — delegates to [`crate::api::traits::Processor::describe`].
    pub fn describe_processor(processor: &dyn crate::api::traits::Processor) -> &'static str {
        processor.describe()
    }

    /// Validate a [`ResilienceConfig`] using the [`Validator`] contract.
    pub fn validate_resilience_config(config: &ResilienceConfig) -> Result<(), String> {
        config.validate()
    }
}

/// Free-function shims for backward compatibility — delegates to [`TransportSvc`] methods.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    // @allow: saf_no_wrapper_methods — adds package name and version, not pure delegation
    TransportSvc::create_config_builder()
}

/// Free-function shim — delegates to [`TransportSvc::create_transport_from_config`].
pub fn create_transport_from_config(
    config: &GrpcChannelConfig,
) -> Result<Arc<dyn GrpcEgress>, GrpcChannelConfigError> {
    TransportSvc::create_transport_from_config(config)
}

/// Free-function shim — delegates to [`TransportSvc::create_tonic_client_from_config`].
pub fn create_tonic_client_from_config(
    config: &GrpcChannelConfig,
) -> Result<TonicGrpcClient, GrpcChannelConfigError> {
    TransportSvc::create_tonic_client_from_config(config)
}

/// Free-function shim — delegates to [`TransportSvc::describe_processor`].
pub fn describe_processor(processor: &dyn crate::api::traits::Processor) -> &'static str {
    TransportSvc::describe_processor(processor)
}

/// Free-function shim — delegates to [`TransportSvc::validate_resilience_config`].
pub fn validate_resilience_config(config: &ResilienceConfig) -> Result<(), String> {
    TransportSvc::validate_resilience_config(config)
}
