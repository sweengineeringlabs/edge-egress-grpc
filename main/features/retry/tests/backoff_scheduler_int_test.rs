//! Coverage stub for `src/api/backoff/backoff_scheduler.rs`.
//!
//! `BackoffScheduler` trait is `pub(crate)` — not part of the public API.
//! The backoff output is publicly accessible as `BackoffSchedule`.

use std::time::Duration;
use swe_edge_egress_grpc_retry::BackoffSchedule;

/// @covers: BackoffScheduler (internal) — BackoffSchedule is the public output
#[test]
fn retry_trait_backoff_scheduler_output_is_accessible_int_test() {
    let schedule = BackoffSchedule::from_duration(Duration::from_millis(100));
    assert_eq!(schedule.sleep, Duration::from_millis(100));
}
