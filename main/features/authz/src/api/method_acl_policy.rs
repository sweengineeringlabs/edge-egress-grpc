//! Struct declaration and constructors for [`MethodAclPolicy`].

use crate::api::MethodAclConfig;

/// Policy that consults a [`MethodAclConfig`] keyed on the caller's CN.
#[derive(Debug, Clone)]
pub struct MethodAclPolicy {
    pub(crate) config: MethodAclConfig,
}

impl MethodAclPolicy {
    /// Construct from an ACL config.
    pub fn from_config(config: MethodAclConfig) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_config
    #[test]
    fn test_from_config_creates_policy() {
        let _ = MethodAclPolicy::from_config(MethodAclConfig::default());
    }
}
