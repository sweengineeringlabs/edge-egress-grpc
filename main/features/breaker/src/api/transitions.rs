//! Interface contract for the breaker state-transition logic.
//!
//! The implementation lives in `core::transitions`.  This file
//! holds the trait that `core::transitions` programs against —
//! satisfies the layer-boundary check that every core/ submodule
//! has an api/ counterpart.

use crate::api::breaker_client::BreakerNode;
use crate::api::breaker_config::GrpcBreakerConfig;
use crate::api::breaker_state::{Admission, Outcome};

/// Interface for the breaker's state-transition primitives.
/// Implemented as free functions in `core::transitions`; this
/// trait exists for the SEA layer-boundary check and as
/// documentation of the contract.  Free-function impls don't
/// instantiate the trait (Rust has no way to "implement" a
/// trait of associated functions for a module), so the trait
/// itself is unused at the type-system level — the
/// `#[allow(dead_code)]` is intentional.
#[allow(dead_code)]
pub(crate) trait BreakerTransitions {
    /// Decide whether to admit a new request.
    fn admit(node: &mut BreakerNode, config: &GrpcBreakerConfig) -> Admission;

    /// Record the outcome of a dispatched request.
    fn record(node: &mut BreakerNode, config: &GrpcBreakerConfig, outcome: Outcome);
}

#[cfg(test)]
mod tests {
    use crate::api::breaker_client::BreakerNode;
    use crate::api::breaker_config::GrpcBreakerConfig;
    use crate::api::breaker_state::{Admission, Outcome};
    use crate::core::transitions;

    use super::BreakerTransitions;

    struct ConcreteTransitions;
    impl BreakerTransitions for ConcreteTransitions {
        fn admit(node: &mut BreakerNode, config: &GrpcBreakerConfig) -> Admission {
            transitions::admit(node, config)
        }
        fn record(node: &mut BreakerNode, config: &GrpcBreakerConfig, outcome: Outcome) {
            transitions::record(node, config, outcome)
        }
    }

    #[test]
    fn test_breaker_transitions_admit_returns_proceed_on_fresh_node() {
        let cfg = GrpcBreakerConfig::from_config(
            "failure_threshold = 3\ncool_down_seconds = 10\nhalf_open_probe_count = 1",
        )
        .unwrap();
        let mut node = BreakerNode::new();
        assert_eq!(
            ConcreteTransitions::admit(&mut node, &cfg),
            Admission::Proceed,
            "fresh node must admit new requests",
        );
    }
}
