//! Minimal usage: inspect the default breaker policy.

fn main() {
    let cfg = edge_transport_grpc_egress_breaker::GrpcBreakerConfig::default();
    println!(
        "edge_transport_grpc_egress_breaker default: \
         failure_threshold={}, cool_down_seconds={}, half_open_probe_count={}",
        cfg.failure_threshold, cfg.cool_down_seconds, cfg.half_open_probe_count,
    );
}
