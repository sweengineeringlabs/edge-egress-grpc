//! Thread-safe circuit breaker with configurable half-open probe count.
//!
//! State machine: Closed → Open → HalfOpen → Closed (or back to Open).
//!
//! **Closed** — normal operation. Failures are counted; when
//! `failure_threshold` consecutive failures accumulate, the breaker
//! opens.
//!
//! **Open** — requests are rejected immediately without touching the
//! downstream.  After `cool_down` elapses, the breaker enters HalfOpen.
//!
//! **HalfOpen** — a probe window.  Each probe success increments a
//! counter; when `half_open_probe_count` consecutive successes are
//! recorded the breaker closes.  Any single failure re-opens it.

use std::sync::Mutex;
use std::time::{Duration, Instant};

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Closed,
    Open { opened_at: Instant },
    HalfOpen,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Closed   => write!(f, "Closed"),
            State::Open { .. } => write!(f, "Open"),
            State::HalfOpen => write!(f, "HalfOpen"),
        }
    }
}

#[derive(Debug)]
struct Inner {
    state:                State,
    consecutive_failures: u32,
    half_open_successes:  u32,
}

// ── CircuitBreaker ────────────────────────────────────────────────────────────

/// Circuit breaker protecting an outbound gRPC transport.
///
/// Construct via [`CircuitBreaker::new`].  Thread-safe via an internal
/// `Mutex<Inner>`.  All operations complete in O(1).
#[derive(Debug)]
pub struct CircuitBreaker {
    inner:                  Mutex<Inner>,
    failure_threshold:      u32,
    cool_down:              Duration,
    half_open_probe_count:  u32,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    ///
    /// - `failure_threshold`: consecutive post-retry failures before opening.
    ///   `0` disables the breaker entirely (always closed).
    /// - `cool_down`: how long the circuit stays open before probing.
    /// - `half_open_probe_count`: consecutive successes required in HalfOpen
    ///   to close the breaker.  Must be >= 1.
    pub fn new(failure_threshold: u32, cool_down: Duration, half_open_probe_count: u32) -> Self {
        Self {
            inner: Mutex::new(Inner {
                state:                State::Closed,
                consecutive_failures: 0,
                half_open_successes:  0,
            }),
            failure_threshold,
            cool_down,
            half_open_probe_count: half_open_probe_count.max(1),
        }
    }

    /// `true` when the breaker is in Open state (requests should be rejected).
    pub fn is_open(&self) -> bool {
        let mut g = self.inner.lock().expect("circuit breaker lock poisoned");
        self.maybe_advance_to_half_open(&mut g);
        matches!(g.state, State::Open { .. })
    }

    /// Record a successful call outcome.
    pub fn record_success(&self) {
        let mut g = self.inner.lock().expect("circuit breaker lock poisoned");
        self.maybe_advance_to_half_open(&mut g);
        match g.state {
            State::Closed => {
                g.consecutive_failures = 0;
            }
            State::HalfOpen => {
                g.half_open_successes += 1;
                if g.half_open_successes >= self.half_open_probe_count {
                    g.state               = State::Closed;
                    g.consecutive_failures = 0;
                    g.half_open_successes  = 0;
                }
            }
            State::Open { .. } => {}
        }
    }

    /// Record a failed call outcome.
    pub fn record_failure(&self) {
        if self.failure_threshold == 0 {
            return;
        }
        let mut g = self.inner.lock().expect("circuit breaker lock poisoned");
        self.maybe_advance_to_half_open(&mut g);
        match g.state {
            State::Closed => {
                g.consecutive_failures += 1;
                if g.consecutive_failures >= self.failure_threshold {
                    g.state              = State::Open { opened_at: Instant::now() };
                    g.half_open_successes = 0;
                }
            }
            State::HalfOpen => {
                // A probe failure re-opens immediately.
                g.state               = State::Open { opened_at: Instant::now() };
                g.half_open_successes = 0;
            }
            State::Open { .. } => {}
        }
    }

    fn maybe_advance_to_half_open(&self, g: &mut Inner) {
        if let State::Open { opened_at } = g.state {
            if opened_at.elapsed() >= self.cool_down {
                g.state               = State::HalfOpen;
                g.consecutive_failures = 0;
                g.half_open_successes  = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cb(threshold: u32) -> CircuitBreaker {
        CircuitBreaker::new(threshold, Duration::from_secs(60), 1)
    }

    /// @covers: CircuitBreaker::new — starts Closed, not open.
    #[test]
    fn test_new_starts_closed() {
        assert!(!cb(5).is_open());
    }

    /// @covers: record_failure — opens after threshold consecutive failures.
    #[test]
    fn test_opens_after_threshold_failures() {
        let c = cb(3);
        c.record_failure();
        c.record_failure();
        assert!(!c.is_open(), "should still be closed after 2 failures");
        c.record_failure();
        assert!(c.is_open(), "should be open after 3 failures");
    }

    /// @covers: record_success — resets consecutive failure count in Closed.
    #[test]
    fn test_success_resets_consecutive_failures() {
        let c = cb(3);
        c.record_failure();
        c.record_failure();
        c.record_success();
        c.record_failure();
        assert!(!c.is_open(), "success must reset the counter");
    }

    /// @covers: failure_threshold = 0 — disables the breaker.
    #[test]
    fn test_threshold_zero_never_opens() {
        let c = cb(0);
        for _ in 0..100 { c.record_failure(); }
        assert!(!c.is_open(), "threshold=0 breaker must never open");
    }

    /// @covers: HalfOpen — transitions from Open after cool_down elapses.
    #[test]
    fn test_transitions_to_half_open_after_cool_down() {
        let c = CircuitBreaker::new(1, Duration::from_millis(1), 1);
        c.record_failure();
        assert!(c.is_open());
        std::thread::sleep(Duration::from_millis(5));
        assert!(!c.is_open(), "should advance to HalfOpen and no longer reject");
    }

    /// @covers: HalfOpen — single probe success closes the breaker.
    #[test]
    fn test_single_probe_success_closes_when_probe_count_is_one() {
        let c = CircuitBreaker::new(1, Duration::from_millis(1), 1);
        c.record_failure();
        std::thread::sleep(Duration::from_millis(5));
        c.record_success();
        assert!(!c.is_open(), "breaker should be Closed after probe success");
    }

    /// @covers: HalfOpen — probe failure re-opens the breaker.
    #[test]
    fn test_probe_failure_reopens_the_breaker() {
        let c = CircuitBreaker::new(1, Duration::from_millis(1), 1);
        c.record_failure();
        std::thread::sleep(Duration::from_millis(5));
        // Advance to HalfOpen.
        assert!(!c.is_open());
        // A failure in HalfOpen re-opens.
        c.record_failure();
        assert!(c.is_open(), "probe failure must re-open the breaker");
    }

    /// @covers: half_open_probe_count > 1 — requires N consecutive successes.
    #[test]
    fn test_multi_probe_requires_n_consecutive_successes() {
        let c = CircuitBreaker::new(1, Duration::from_millis(1), 3);
        c.record_failure();
        std::thread::sleep(Duration::from_millis(5));
        // Two probe successes: not yet closed.
        c.record_success();
        c.record_success();
        assert!(!c.is_open(), "2 of 3 probes done — should be HalfOpen, not Open");
        // Force-check still HalfOpen (not Closed yet).
        {
            let g = c.inner.lock().unwrap();
            assert_eq!(g.state, State::HalfOpen, "should still be HalfOpen");
        }
        // Third success closes it.
        c.record_success();
        assert!(!c.is_open());
        {
            let g = c.inner.lock().unwrap();
            assert_eq!(g.state, State::Closed, "should be Closed after 3 probes");
        }
    }

    /// @covers: half_open_probe_count > 1 — probe failure resets successes.
    #[test]
    fn test_probe_failure_resets_half_open_success_count() {
        let c = CircuitBreaker::new(1, Duration::from_millis(1), 3);
        c.record_failure();
        std::thread::sleep(Duration::from_millis(5));
        c.record_success(); // 1 of 3
        c.record_failure(); // re-opens, resets count
        assert!(c.is_open());
    }
}
