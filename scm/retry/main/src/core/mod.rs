//! Core layer — decorator impl + backoff schedule logic + processor.

pub(crate) mod backoff_schedule;
pub(crate) mod backoff_scheduler;
pub(crate) mod grpc_retry_client;
pub(crate) mod grpc_retry_config;
pub(crate) mod grpc_retry_config_builder;
pub(crate) mod resource_exhausted_context;
pub(crate) mod retry_decision;
pub(crate) mod retry_egress;
pub(crate) mod traits;
