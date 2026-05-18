//! Interface counterpart for `core/factory` — assembly contract documentation.
//!
//! `core/factory::assemble` produces an `Arc<dyn GrpcOutbound>` that is
//! optionally wrapped in retry + circuit-breaker layers when
//! [`swe_edge_egress_grpc::GrpcChannelConfig::resilience`] is `Some`.

#[cfg(test)]
mod tests {
    use swe_edge_egress_grpc::GrpcOutbound;

    /// Compile-time guard: `GrpcOutbound` must remain object-safe for
    /// `Arc<dyn GrpcOutbound>` to work as the assembled transport type.
    #[test]
    fn test_grpc_outbound_is_object_safe_for_arc_dyn() {
        fn _assert(_: &dyn GrpcOutbound) {}
    }
}
