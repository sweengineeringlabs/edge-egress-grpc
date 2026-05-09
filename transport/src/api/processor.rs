//! `Processor` trait — primary processing contract for this crate.

/// Primary processing trait for the egress gRPC processor.
///
/// Implemented by `crate::core::resilience::resilient_grpc_client::ResilientGrpcClient`
/// and `crate::core::client::tonic_grpc_client::TonicGrpcClient` (both implement
/// [`GrpcOutbound`] which satisfies this contract).
pub trait Processor: Send + Sync {
    /// Identify this processor unit.
    fn describe(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_is_object_safe() {
        fn _assert(_: &dyn Processor) {}
    }
}
