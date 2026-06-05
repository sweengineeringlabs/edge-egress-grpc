//! Coverage stub for `src/api/retry/retry_egress.rs`.
//!
//! `RetryEgress` trait is `pub(crate)` — not part of the public API.
//! The public surface that depends on it is `GrpcRetryClient`.

use swe_edge_egress_grpc_retry::{GrpcRetryClient, GrpcRetryConfig};

/// @covers: RetryEgress (internal) — GrpcRetryClient is the public face
#[test]
fn retry_trait_retry_egress_grpc_retry_client_is_accessible_int_test() {
    let cfg = GrpcRetryConfig::default();
    let client = GrpcRetryClient::new((), cfg);
    assert!(client.config().max_attempts >= 1);
}
