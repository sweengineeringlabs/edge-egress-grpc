//! Error variants emitted by the outbound bearer interceptor.

/// Reasons the outbound bearer interceptor fails to mint a token.
#[derive(Debug, thiserror::Error)]
pub enum BearerAuthError {
    /// The outbound interceptor failed to encode/sign the JWT.
    #[error("failed to mint bearer token")]
    SignFailed(#[source] jsonwebtoken::errors::Error),

    /// System clock is before the Unix epoch — should never happen.
    #[error("system clock is before Unix epoch")]
    InvalidSystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: BearerAuthError implements std::error::Error.
    #[test]
    fn test_bearer_auth_error_implements_std_error() {
        let e = BearerAuthError::InvalidSystemTime;
        let _: &dyn std::error::Error = &e;
        assert!(e.to_string().contains("clock"));
    }
}
