//! Core layer — breaker state-transition logic + decorator impl.

pub(crate) mod breaker_decorator;
pub(crate) mod breaker_egress;
pub(crate) mod breaker_node;
pub(crate) mod breaker_transition;
pub(crate) mod config_builder_provider;
pub(crate) mod default_processor;
pub(crate) mod default_validator;
pub(crate) mod failure_classifier;
pub(crate) mod grpc_breaker_client;
pub(crate) mod grpc_breaker_config;
