//! `GrpcOutboundResult` — result type alias for gRPC outbound operations.

use crate::api::port::grpc::grpc_outbound_error::GrpcOutboundError;

/// Result type for gRPC outbound operations.
pub type GrpcOutboundResult<T> = Result<T, GrpcOutboundError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_outbound_result_ok_variant_is_constructable() {
        let r: GrpcOutboundResult<u32> = Ok(42);
        let Ok(v) = r else { panic!("expected Ok") };
        assert_eq!(v, 42);
    }

    #[test]
    fn test_grpc_outbound_result_err_variant_carries_error() {
        use crate::api::value_object::GrpcStatusCode;
        let r: GrpcOutboundResult<u32> = Err(GrpcOutboundError::Status(
            GrpcStatusCode::NotFound,
            "gone".into(),
        ));
        assert!(r.is_err());
    }
}
