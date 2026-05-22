//! Interface counterpart for `core/factory` — assembly contract documentation.
//!
//! `core/factory::assemble` produces an `Arc<dyn GrpcEgress>` that is
//! optionally wrapped in retry + circuit-breaker layers when
//! [`swe_edge_egress_grpc::GrpcChannelConfig::resilience`] is `Some`.

#[cfg(test)]
mod tests {
    use swe_edge_egress_grpc::GrpcEgress;

    /// Compile-time guard: `GrpcEgress` must remain object-safe for
    /// `Arc<dyn GrpcEgress>` to work as the assembled transport type.
    #[test]
    fn test_grpc_egress_is_object_safe_for_arc_dyn() {
        fn _assert(_: &dyn GrpcEgress) {}
    }
}
