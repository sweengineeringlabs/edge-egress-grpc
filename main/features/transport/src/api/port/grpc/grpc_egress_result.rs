//! `GrpcEgressResult` — result type alias for gRPC outbound operations.

use crate::api::port::grpc::grpc_egress_error::GrpcEgressError;

/// Result type for gRPC outbound operations.
pub type GrpcEgressResult<T> = Result<T, GrpcEgressError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_egress_result_ok_variant_is_constructable() {
        let r: GrpcEgressResult<u32> = Ok(42);
        let Ok(v) = r else { panic!("expected Ok") };
        assert_eq!(v, 42);
    }

    #[test]
    fn test_grpc_egress_result_err_variant_carries_error() {
        use crate::api::value::GrpcStatusCode;
        let r: GrpcEgressResult<u32> = Err(GrpcEgressError::Status(
            GrpcStatusCode::NotFound,
            "gone".into(),
        ));
        assert!(r.is_err());
    }
}
