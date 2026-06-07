//! Validator implementation for BearerEgressConfig.

use crate::api::traits::validator::Validator;
use crate::api::types::bearer_egress_config::BearerEgressConfig;

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
