//! Types — behavioural type declarations whose impl blocks live in `core/`.

pub(crate) mod admission;
pub(crate) mod admit_request;
pub(crate) mod admit_response;
pub mod application_config_builder;
pub(crate) mod breaker_node;
pub(crate) mod breaker_state;
pub mod config_builder_request;
pub mod config_validation_request;
pub mod describe_request;
pub mod describe_response;
pub mod grpc_breaker_client;
pub mod grpc_breaker_config;
pub mod grpc_breaker_svc;
pub mod observe_state_request;
pub mod observe_state_response;
pub(crate) mod outcome;
pub(crate) mod record_outcome_request;
pub(crate) mod record_outcome_response;
pub mod wrap_breaker_request;

pub use application_config_builder::ApplicationConfigBuilder;
pub use breaker_state::BreakerState;
pub use config_builder_request::ConfigBuilderRequest;
pub use config_validation_request::ConfigValidationRequest;
pub use describe_request::DescribeRequest;
pub use describe_response::DescribeResponse;
pub use grpc_breaker_client::GrpcBreakerClient;
pub use grpc_breaker_config::GrpcBreakerConfig;
pub use grpc_breaker_svc::GrpcBreakerSvc;
pub use observe_state_request::ObserveStateRequest;
pub use observe_state_response::ObserveStateResponse;
pub use wrap_breaker_request::WrapBreakerRequest;

pub(crate) use admission::Admission;
pub(crate) use admit_request::AdmitRequest;
pub(crate) use admit_response::AdmitResponse;
pub(crate) use breaker_node::BreakerNode;
pub(crate) use outcome::Outcome;
pub(crate) use record_outcome_request::RecordOutcomeRequest;
pub(crate) use record_outcome_response::RecordOutcomeResponse;
