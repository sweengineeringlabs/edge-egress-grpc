//! Configuration for the outbound bearer interceptor.

use serde::{Deserialize, Serialize};

use crate::api::bearer::bearer_secret::BearerSecret;

/// Outbound (client) bearer config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearerEgressConfig {
    /// Algorithm + key material to sign the JWT.  HS256 expects raw
    /// bytes; RS256 expects PEM.
    pub secret: BearerSecret,
    /// JWT `iss` claim.
    pub issuer: String,
    /// JWT `aud` claim.
    pub audience: String,
    /// JWT `sub` claim.
    pub subject: String,
    /// Lifetime of minted tokens in seconds.
    pub lifetime_seconds: u64,
}
