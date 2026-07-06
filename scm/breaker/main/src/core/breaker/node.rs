//! Constructor for [`BreakerNode`].

use crate::api::{BreakerNode, BreakerState};

impl BreakerNode {
    pub(crate) fn new() -> Self {
        Self {
            state: BreakerState::Closed,
            consecutive_failures: 0,
            consecutive_successes: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_starts_closed_with_zero_counters() {
        let node = BreakerNode::new();
        assert!(matches!(node.state, BreakerState::Closed));
        assert_eq!(node.consecutive_failures, 0);
        assert_eq!(node.consecutive_successes, 0);
    }
}
