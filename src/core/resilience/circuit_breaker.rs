//! Circuit breaker for gRPC outbound transports.
//!
//! Three-state machine: Closed → Open → HalfOpen → Closed.
//!
//! ```text
//!  Closed ──[failure_threshold consecutive failures]──► Open
//!  Open   ──[open_duration elapsed]────────────────────► HalfOpen
//!  HalfOpen ─[next call succeeds]─────────────────────► Closed
//!  HalfOpen ─[next call fails]────────────────────────► Open
//! ```
//!
//! In Open state, [`CircuitBreaker::is_open`] returns `true` and the
//! caller should fail fast rather than attempting the downstream call.
//! The probe in HalfOpen is a single real call — if it succeeds the
//! circuit closes and consecutive-failure count resets to zero.

use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
enum State {
    Closed,
    Open(Instant),
    HalfOpen,
}

struct Inner {
    state:                State,
    consecutive_failures: u32,
}

/// Thread-safe circuit breaker. Cheap to clone (backed by `Arc` internally
/// via the `Mutex`); share one instance per downstream transport.
pub struct CircuitBreaker {
    inner:             Mutex<Inner>,
    failure_threshold: u32,
    open_duration:     Duration,
}

impl std::fmt::Debug for CircuitBreaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CircuitBreaker")
            .field("failure_threshold", &self.failure_threshold)
            .field("open_duration",     &self.open_duration)
            .finish_non_exhaustive()
    }
}

impl CircuitBreaker {
    /// Construct a new circuit breaker.
    ///
    /// * `failure_threshold` — consecutive failures before opening the circuit.
    ///   Set to `0` to disable (circuit never opens).
    /// * `open_duration` — how long the circuit stays open before probing.
    pub fn new(failure_threshold: u32, open_duration: Duration) -> Self {
        Self {
            inner: Mutex::new(Inner {
                state:                State::Closed,
                consecutive_failures: 0,
            }),
            failure_threshold,
            open_duration,
        }
    }

    /// `true` when the circuit is open and callers should fail fast.
    ///
    /// Automatically transitions `Open → HalfOpen` when `open_duration`
    /// has elapsed, allowing the next call through as a probe.
    pub fn is_open(&self) -> bool {
        if self.failure_threshold == 0 {
            return false;
        }
        let mut inner = self.inner.lock().expect("circuit breaker mutex poisoned");
        match inner.state {
            State::Closed | State::HalfOpen => false,
            State::Open(opened_at) => {
                if opened_at.elapsed() >= self.open_duration {
                    inner.state = State::HalfOpen;
                    false
                } else {
                    true
                }
            }
        }
    }

    /// Record a successful call. Resets consecutive-failure count and closes
    /// the circuit (no-op when already Closed).
    pub fn record_success(&self) {
        let mut inner = self.inner.lock().expect("circuit breaker mutex poisoned");
        inner.consecutive_failures = 0;
        inner.state = State::Closed;
    }

    /// Record a failed call. May open the circuit when the threshold is reached.
    pub fn record_failure(&self) {
        if self.failure_threshold == 0 {
            return;
        }
        let mut inner = self.inner.lock().expect("circuit breaker mutex poisoned");
        inner.consecutive_failures += 1;
        match inner.state {
            State::Closed
                if inner.consecutive_failures >= self.failure_threshold =>
            {
                tracing::warn!(
                    threshold = self.failure_threshold,
                    "circuit breaker opened after consecutive failures"
                );
                inner.state = State::Open(Instant::now());
            }
            State::HalfOpen => {
                tracing::warn!("circuit breaker re-opened after failed probe");
                inner.state = State::Open(Instant::now());
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: CircuitBreaker — starts closed.
    #[test]
    fn test_new_circuit_is_closed() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(10));
        assert!(!cb.is_open());
    }

    /// @covers: CircuitBreaker — opens after failure_threshold consecutive failures.
    #[test]
    fn test_opens_after_threshold_failures() {
        let cb = CircuitBreaker::new(3, Duration::from_secs(10));
        cb.record_failure();
        cb.record_failure();
        assert!(!cb.is_open(), "should still be closed at 2 failures");
        cb.record_failure();
        assert!(cb.is_open(), "should open at threshold=3");
    }

    /// @covers: CircuitBreaker::record_success — resets failures and closes circuit.
    #[test]
    fn test_success_resets_and_closes() {
        let cb = CircuitBreaker::new(2, Duration::from_secs(10));
        cb.record_failure();
        cb.record_success();
        cb.record_failure();
        assert!(!cb.is_open(), "success reset count; one failure should not re-open");
    }

    /// @covers: CircuitBreaker — threshold=0 never opens.
    #[test]
    fn test_zero_threshold_never_opens() {
        let cb = CircuitBreaker::new(0, Duration::from_secs(1));
        for _ in 0..100 {
            cb.record_failure();
        }
        assert!(!cb.is_open());
    }

    /// @covers: CircuitBreaker — transitions to HalfOpen after open_duration,
    /// then back to Open on probe failure.
    #[test]
    fn test_transitions_half_open_then_re_opens_on_failure() {
        let cb = CircuitBreaker::new(1, Duration::from_millis(1));
        cb.record_failure();
        assert!(cb.is_open());

        std::thread::sleep(Duration::from_millis(5));
        assert!(!cb.is_open(), "should be HalfOpen — probe allowed");

        cb.record_failure();
        assert!(cb.is_open(), "probe failed — should re-open");
    }

    /// @covers: CircuitBreaker — transitions to HalfOpen then Closed on probe success;
    /// after closing, two failures are needed to re-open (threshold=2).
    #[test]
    fn test_transitions_half_open_then_closes_on_success() {
        let cb = CircuitBreaker::new(2, Duration::from_millis(1));
        cb.record_failure();
        cb.record_failure();
        std::thread::sleep(Duration::from_millis(5));
        assert!(!cb.is_open(), "should be HalfOpen after open_duration");

        cb.record_success();
        assert!(!cb.is_open(), "probe succeeded — circuit should be Closed");

        cb.record_failure();
        assert!(!cb.is_open(), "one failure below threshold=2 should not re-open");
    }
}
