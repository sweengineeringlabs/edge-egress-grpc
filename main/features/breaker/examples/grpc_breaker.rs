//! Minimal usage: inspect the default breaker policy.

fn main() {
    let cfg = swe_edge_egress_grpc_breaker::GrpcBreakerConfig::default();
    println!(
        "swe_edge_egress_grpc_breaker default: \
         failure_threshold={}, cool_down_seconds={}, half_open_probe_count={}",
        cfg.failure_threshold, cfg.cool_down_seconds, cfg.half_open_probe_count,
    );
}
