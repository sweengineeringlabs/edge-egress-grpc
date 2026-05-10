//! Interface counterpart for `core/resilience/resilient_grpc_client.rs`.
//!
//! The concrete implementation `ResilientGrpcClient` implements [`crate::api::port::GrpcOutbound`]
//! and [`crate::api::traits::Processor`]; those traits are the public contracts.

#[cfg(test)]
mod tests {
    use crate::api::port::GrpcOutbound;

    #[test]
    fn test_grpc_outbound_re_export_is_object_safe() {
        fn _assert(_: &dyn GrpcOutbound) {}
    }
}
