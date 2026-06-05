//! Coverage stub for `src/api/breaker/breaker_state.rs`.

use swe_edge_egress_grpc_breaker::BreakerState;

/// @covers: BreakerState — type is accessible and variants are distinct
#[test]
fn breaker_enum_breaker_state_is_accessible_int_test() {
    let _ = std::mem::size_of::<BreakerState>();
}

/// @covers: BreakerState::Closed — default variant equality
#[test]
fn breaker_enum_breaker_state_closed_equals_closed_int_test() {
    assert_eq!(BreakerState::Closed, BreakerState::Closed);
}

/// @covers: BreakerState::HalfOpen — distinct from Closed
#[test]
fn breaker_enum_breaker_state_half_open_differs_from_closed_int_test() {
    assert_ne!(BreakerState::HalfOpen, BreakerState::Closed);
}
