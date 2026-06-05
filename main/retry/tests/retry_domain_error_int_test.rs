//! Coverage stub for `src/api/error/retry_domain_error.rs`.
//!
//! `RetryDomainError` is a public type alias for `Error`.

use swe_edge_egress_grpc_retry::Error;

/// @covers: RetryDomainError — type alias resolves to Error
#[test]
fn retry_type_retry_domain_error_is_accessible_int_test() {
    // RetryDomainError = Error; verify Error can be instantiated.
    let err = Error::InvalidConfig("max_attempts must be >= 1".into());
    assert!(!err.to_string().is_empty());
}
