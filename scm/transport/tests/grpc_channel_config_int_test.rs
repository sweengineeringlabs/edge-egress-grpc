#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for `GrpcChannelConfig` and `TransportSvc::create_transport_from_config`.
//!
//! Verifies the **fail-closed TLS-by-default invariant** at the
//! public-facade boundary via the `TransportSvc::create_transport_from_config` factory.

use std::sync::Arc;

use swe_edge_egress_grpc_transport::{
    CompressionMode, GrpcChannelConfig, GrpcChannelConfigError, GrpcEgress, GrpcEgressError,
    GrpcEgressInterceptor, GrpcEgressInterceptorChain, GrpcRequest, GrpcResponse, GrpcStatusCode,
    TraceContextInterceptor, TransportSvc,
};

fn ensure_rustls_provider() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

/// @covers: GrpcChannelConfig::default — tls_required is `true`.
#[test]
fn transport_struct_channel_config_default_requires_tls_int_test() {
    let cfg = GrpcChannelConfig::default();
    assert!(cfg.tls_required, "TLS-by-default invariant must hold");
}

/// @covers: TransportSvc::create_transport_from_config — plaintext endpoint rejected when tls_required.
#[test]
fn transport_struct_channel_config_from_config_rejects_plaintext_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://localhost:50051");
    let result = TransportSvc::create_transport_from_config(&cfg);
    match result {
        Err(GrpcChannelConfigError::PlaintextRejected(endpoint)) => {
            assert!(
                endpoint.contains("localhost"),
                "endpoint in error: {endpoint}"
            );
        }
        Err(GrpcChannelConfigError::Config(msg)) => panic!("unexpected Config error: {msg}"),
        Ok(_) => panic!("must reject plaintext when tls_required=true"),
    }
}

/// @covers: TransportSvc::create_transport_from_config — plaintext accepted with allow_plaintext().
#[test]
fn transport_struct_channel_config_from_config_accepts_plaintext_with_opt_in_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("http://localhost:50051").allow_plaintext();
    assert!(TransportSvc::create_transport_from_config(&cfg).is_ok());
}

/// @covers: TransportSvc::create_transport_from_config — https endpoint accepted.
#[test]
fn transport_struct_channel_config_from_config_accepts_https_int_test() {
    ensure_rustls_provider();
    let cfg = GrpcChannelConfig::new("https://example.com:443");
    assert!(TransportSvc::create_transport_from_config(&cfg).is_ok());
}

/// @covers: GrpcEgressInterceptorChain — accepts a TraceContextInterceptor.
#[test]
fn transport_struct_channel_config_with_interceptors_accepts_chain_int_test() {
    let chain =
        GrpcEgressInterceptorChain::new().push(Arc::new(TraceContextInterceptor::pass_through()));
    assert_eq!(chain.len(), 1);
}

/// @covers: GrpcEgressInterceptorChain — before_call short-circuits on first failure.
#[tokio::test]
async fn transport_struct_channel_config_interceptor_short_circuits_int_test() {
    use std::time::Duration;

    ensure_rustls_provider();

    struct Deny;
    impl GrpcEgressInterceptor for Deny {
        fn before_call(&self, _: &mut GrpcRequest) -> Result<(), GrpcEgressError> {
            Err(GrpcEgressError::Status(
                GrpcStatusCode::PermissionDenied,
                "denied".into(),
            ))
        }
        fn after_call(&self, _: &mut GrpcResponse) -> Result<(), GrpcEgressError> {
            Ok(())
        }
    }

    let cfg = GrpcChannelConfig::new("http://127.0.0.1:1").allow_plaintext();
    let base = TransportSvc::create_transport_from_config(&cfg).expect("transport");
    let chain = GrpcEgressInterceptorChain::new().push(Arc::new(Deny));

    struct WithChain {
        inner: Arc<dyn GrpcEgress>,
        chain: GrpcEgressInterceptorChain,
    }
    impl GrpcEgress for WithChain {
        fn call_unary(
            &self,
            mut req: GrpcRequest,
        ) -> futures::future::BoxFuture<
            '_,
            swe_edge_egress_grpc_transport::GrpcEgressResult<GrpcResponse>,
        > {
            Box::pin(async move {
                self.chain.run_before(&mut req)?;
                self.inner.call_unary(req).await
            })
        }
        fn health_check(
            &self,
        ) -> futures::future::BoxFuture<'_, swe_edge_egress_grpc_transport::GrpcEgressResult<()>>
        {
            self.inner.health_check()
        }
    }

    let client = WithChain { inner: base, chain };
    let req = GrpcRequest::new("svc/Method", vec![1, 2, 3], Duration::from_secs(1));
    match client.call_unary(req).await {
        Err(GrpcEgressError::Status(GrpcStatusCode::PermissionDenied, msg)) => {
            assert_eq!(msg, "denied");
        }
        other => panic!("expected PermissionDenied; got {other:?}"),
    }
}

/// @covers: GrpcChannelConfig::with_compression — sets mode.
#[test]
fn transport_struct_channel_config_with_compression_overrides_mode_int_test() {
    let cfg = GrpcChannelConfig::new("http://localhost:0").with_compression(CompressionMode::Gzip);
    assert_eq!(cfg.compression, CompressionMode::Gzip);
}
