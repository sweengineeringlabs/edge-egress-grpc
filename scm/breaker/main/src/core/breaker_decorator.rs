//! `impl BreakerDecorator for DefaultBreakerDecorator`.

use crate::api::{BreakerDecorator, BreakerDomainError, GrpcBreakerClient, WrapBreakerRequest};

/// Default [`BreakerDecorator`] implementation — wraps `inner` with
/// [`GrpcBreakerClient::new`].
pub(crate) struct DefaultBreakerDecorator;

impl<T> BreakerDecorator<T> for DefaultBreakerDecorator {
    fn wrap(&self, req: WrapBreakerRequest<T>) -> Result<GrpcBreakerClient<T>, BreakerDomainError> {
        Ok(GrpcBreakerClient::new(req.inner, req.config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::GrpcBreakerConfig;

    #[test]
    fn test_wrap_produces_a_client_with_the_supplied_config() {
        let resp = DefaultBreakerDecorator
            .wrap(WrapBreakerRequest {
                inner: (),
                config: GrpcBreakerConfig {
                    failure_threshold: 9,
                    cool_down_seconds: 5,
                    half_open_probe_count: 1,
                },
            })
            .expect("wrap is infallible");
        assert_eq!(resp.config().failure_threshold, 9);
    }
}
