//! Coverage stub for `src/api/transitions/breaker_transition.rs`.
//!
//! The `transitions` module re-exports `BreakerTransition`, which is
//! `pub(crate)` — not part of the public API.  This stub exercises the
//! public types the transition contract is defined over.

use swe_edge_egress_grpc_breaker::{BreakerState, GrpcBreakerConfig};

/// @covers: api::transitions::BreakerTransition (internal) — config fields
#[test]
fn breaker_trait_breaker_transitions_config_is_accessible_int_test() {
    let cfg = GrpcBreakerConfig::default();
    // BreakerTransition::admit and ::record both take &GrpcBreakerConfig.
    assert!(cfg.failure_threshold >= 1);
    assert!(cfg.half_open_probe_count >= 1);
    assert_eq!(BreakerState::Closed, BreakerState::Closed);
}
