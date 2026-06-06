//! Minimal usage: inspect the default retry policy.
//!
//! Wrapping a real `GrpcEgress` requires a transport (see
//! `swe-edge-egress-grpc`'s `TonicGrpcClient`); this example shows
//! only the configuration step that doesn't need a server.

fn main() {
    let cfg = swe_edge_egress_grpc_retry::GrpcRetryConfig::default();
    println!(
        "swe_edge_egress_grpc_retry default: \
         max_attempts={}, initial_backoff_ms={}, multiplier={}, jitter={}, max_backoff_ms={}",
        cfg.max_attempts,
        cfg.initial_backoff_ms,
        cfg.backoff_multiplier,
        cfg.jitter_factor,
        cfg.max_backoff_ms,
    );
}
