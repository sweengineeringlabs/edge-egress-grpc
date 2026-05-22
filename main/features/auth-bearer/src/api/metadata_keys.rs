//! Reserved metadata keys used by the bearer interceptors.

/// Standard HTTP/2 / gRPC `authorization` metadata key (lower-case).
pub const AUTHORIZATION_HEADER: &str = "authorization";

/// Internal metadata key under which a successfully validated JWT
/// `sub` claim is republished by [`crate::BearerIngressInterceptor`]
/// for downstream authz policies.  Treated as **trusted** only when
/// set by this interceptor — the inbound interceptor ALWAYS strips
/// any incoming value before it (re-)inserts the verified one.
pub const EXTRACTED_BEARER_SUBJECT: &str = "x-edge-extracted-bearer-subject";

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: AUTHORIZATION_HEADER
    #[test]
    fn test_authorization_header_is_lowercase() {
        assert_eq!(AUTHORIZATION_HEADER, "authorization");
    }

    /// @covers: EXTRACTED_BEARER_SUBJECT
    #[test]
    fn test_extracted_bearer_subject_has_x_edge_prefix() {
        assert!(EXTRACTED_BEARER_SUBJECT.starts_with("x-edge-"));
    }
}
