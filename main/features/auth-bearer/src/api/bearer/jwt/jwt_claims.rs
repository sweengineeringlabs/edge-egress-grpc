//! JWT claims type declaration — the public interface for JWT claims used
//! by bearer interceptors.

use serde::{Deserialize, Serialize};

/// Standard JWT claims set used by the bearer interceptors.
///
/// Custom claims are intentionally not modelled — extending this struct is
/// the migration path when a deployment needs additional fields.
///
/// `exp` and `iat` are seconds since the Unix epoch. The interceptor sets them
/// automatically when minting; you only need to construct this for testing.
///
/// # Examples
///
/// ```rust
/// use swe_edge_egress_grpc_auth_bearer::JwtClaims;
///
/// let claims = JwtClaims {
///     iss: "my-service".to_string(),
///     aud: "upstream".to_string(),
///     sub: "service-account".to_string(),
///     exp: 9_999_999_999,
///     iat: 1_700_000_000,
/// };
///
/// assert_eq!(claims.iss, "my-service");
/// assert!(claims.exp > claims.iat);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// JWT `iss` (issuer) claim.
    pub iss: String,
    /// JWT `aud` (audience) claim.
    pub aud: String,
    /// JWT `sub` (subject) claim.
    pub sub: String,
    /// JWT `exp` (expiration) claim — seconds since Unix epoch.
    pub exp: u64,
    /// JWT `iat` (issued-at) claim — seconds since Unix epoch.
    pub iat: u64,
}
