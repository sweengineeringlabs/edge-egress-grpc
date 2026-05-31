//! JWT claims type declaration — the public interface for JWT claims used
//! by bearer interceptors.

use serde::{Deserialize, Serialize};

/// Standard JWT claims set used by the bearer interceptors.
///
/// Custom claims are intentionally not modelled — extending this struct is
/// the migration path when a deployment needs additional fields.
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
