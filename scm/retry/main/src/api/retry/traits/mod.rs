//! Primary trait declarations for `swe_edge_egress_grpc_retry`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |
//! | [`JitterRng`] | Jitter RNG contract for backoff computation |
//! | [`ConfigBuilderProvider`] | Pre-seeded application config builder contract |
//! | [`BackoffScheduler`] | Backoff computation contract |
//! | [`RetryDecorator`] | Retry-client construction contract |
//! | [`RetryInspector`] | Retry-decision inspection contract |

pub mod backoff_scheduler;
pub mod config_builder_provider;
pub mod jitter_rng;
pub mod processor;
pub mod retry_decorator;
pub mod retry_inspector;
pub mod validator;

pub use backoff_scheduler::BackoffScheduler;
pub use config_builder_provider::ConfigBuilderProvider;
pub use jitter_rng::JitterRng;
pub use processor::Processor;
pub use retry_decorator::RetryDecorator;
pub use retry_inspector::RetryInspector;
pub use validator::Validator;
