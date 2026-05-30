//! Secret material for JWT signing.

use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

/// Symmetric / asymmetric secret material for JWT signing.
///
/// `Hs256` carries a raw byte secret; comparisons MUST go through
/// [`BearerSecret::ct_eq_hs256`] which uses `subtle::ConstantTimeEq`.
/// Asymmetric variants carry PEM-encoded key bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BearerSecret {
    /// HS256 — shared symmetric secret.
    Hs256 {
        /// Raw secret bytes (UTF-8 in TOML; arbitrary bytes via API).
        secret: Vec<u8>,
    },
    /// RS256 — private PEM bytes for signing.
    Rs256 {
        /// PEM-encoded private key.
        #[serde(default)]
        private_pem: Vec<u8>,
    },
}

impl BearerSecret {
    /// Constant-time equality on HS256 secrets.  Returns `false` for
    /// different variants — algorithm-mismatch is never "equal".
    pub fn ct_eq_hs256(&self, other: &Self) -> bool {
        match (self, other) {
            (BearerSecret::Hs256 { secret: a }, BearerSecret::Hs256 { secret: b }) => {
                a.as_slice().ct_eq(b.as_slice()).into()
            }
            _ => false,
        }
    }
}
