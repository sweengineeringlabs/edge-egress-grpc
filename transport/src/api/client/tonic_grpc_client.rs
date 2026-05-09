//! Interface counterpart for `core/client/tonic_grpc_client.rs`.
//!
//! The concrete implementation `TonicGrpcClient` implements [`crate::api::port::GrpcOutbound`]
//! and [`crate::api::traits::Processor`]; those are the public contracts.

pub use crate::api::port::GrpcOutbound;
pub use crate::api::traits::Processor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_outbound_client_interface_is_object_safe() {
        fn _assert(_: &dyn GrpcOutbound) {}
    }
}
