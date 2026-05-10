//! Shared JWT claims schema.

use serde::{Deserialize, Serialize};

/// Standard JWT claims set used by the bearer interceptors.  Custom
/// claims are intentionally not modelled — extending this struct is
/// the migration path when a deployment needs additional fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JwtClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: JwtClaims
    #[test]
    fn test_jwt_claims_struct_is_constructible() {
        let claims = JwtClaims {
            iss: "issuer".into(),
            aud: "audience".into(),
            sub: "user".into(),
            exp: 9999999999,
            iat: 0,
        };
        assert_eq!(claims.sub, "user");
    }
}
