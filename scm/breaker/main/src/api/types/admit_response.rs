//! Response for [`crate::api::traits::BreakerTransition::admit`].

use crate::api::types::admission::Admission;
use crate::api::types::breaker_node::BreakerNode;

/// Output of [`crate::api::traits::BreakerTransition::admit`] — the
/// admission decision and the (possibly promoted) node snapshot.
pub(crate) struct AdmitResponse {
    pub(crate) admission: Admission,
    pub(crate) node: BreakerNode,
}
