//! Configuration for the outbound bearer interceptor.

use serde::{Deserialize, Serialize};

use crate::api::bearer::bearer_secret::BearerSecret;
use crate::api::traits::Validator;

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

impl Validator for BearerEgressConfig {
    fn validate(&self) -> Result<(), String> {
        if self.issuer.is_empty() {
            return Err("issuer must not be empty".into());
        }
        if self.audience.is_empty() {
            return Err("audience must not be empty".into());
        }
        if self.subject.is_empty() {
            return Err("subject must not be empty".into());
        }
        Ok(())
    }
}
