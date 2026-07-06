//! Response for [`crate::api::FailureClassifier::classify`].

use crate::api::Outcome;

/// Output of [`crate::api::FailureClassifier::classify`].
pub struct ClassifyResponse {
    /// The classified outcome.
    pub outcome: Outcome,
}
