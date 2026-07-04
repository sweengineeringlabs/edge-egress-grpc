//! gRPC breaker SAF — factory methods on [`GrpcBreakerSvc`].

use swe_edge_egress_grpc::GrpcEgress;

use crate::api::{BreakerDomainError, ConfigBuilderRequest, WrapBreakerRequest};
use crate::saf::{BreakerDecoratorFactory, ConfigBuilderProviderFactory};

pub(crate) use crate::api::{
    ApplicationConfigBuilder, GrpcBreakerClient, GrpcBreakerConfig, GrpcBreakerSvc,
};

impl GrpcBreakerSvc {
    /// Return a config builder pre-seeded with this crate's name and version.
    pub fn create_config_builder() -> Result<ApplicationConfigBuilder, BreakerDomainError> {
        ConfigBuilderProviderFactory::create().create_config_builder(ConfigBuilderRequest {
            svc: GrpcBreakerSvc,
        })
    }

    /// Wrap `inner` with the supplied breaker policy.
    pub fn wrap_breaker<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
        config: GrpcBreakerConfig,
    ) -> Result<GrpcBreakerClient<T>, BreakerDomainError> {
        BreakerDecoratorFactory::create::<T>().wrap(WrapBreakerRequest { inner, config })
    }

    /// Wrap `inner` with the default breaker policy.
    pub fn create_breaker_client<T: GrpcEgress + Send + Sync + 'static>(
        inner: T,
    ) -> Result<GrpcBreakerClient<T>, BreakerDomainError> {
        Self::wrap_breaker(inner, GrpcBreakerConfig::default())
    }
}
