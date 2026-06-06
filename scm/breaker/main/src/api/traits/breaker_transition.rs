//! Interface contract for the breaker state-transition logic.
//!
//! The implementation lives in `core::breaker_client`.  This file
//! holds the trait that `core::breaker_transition` programs against —
//! satisfies the layer-boundary check that every core/ submodule
//! has an api/ counterpart.

use crate::api::types::breaker_node::BreakerNode;
use crate::api::vo::admission::Admission;
use crate::api::vo::grpc_breaker_config::GrpcBreakerConfig;
use crate::api::vo::outcome::Outcome;

/// Interface for the breaker's state-transition primitives.
/// Implemented as associated functions in `core::breaker_transition`; this
/// trait exists for the SEA layer-boundary check and as
/// documentation of the contract.  Free-function impls don't
/// instantiate the trait (Rust has no way to "implement" a
/// trait of associated functions for a module), so the trait
/// itself is unused at the type-system level.
#[expect(
    dead_code,
    reason = "SEA api/ counterpart — structural anchor, not implemented by any type"
)]
pub trait BreakerTransition {
    /// Decide whether to admit a new request.
    fn admit(node: &mut BreakerNode, config: &GrpcBreakerConfig) -> Admission;

    /// Record the outcome of a dispatched request.
    fn record(node: &mut BreakerNode, config: &GrpcBreakerConfig, outcome: Outcome);
}
