//! `GrpcClientBuilder` — interface counterpart for `core/client/grpc_client_builder.rs`.

/// Builder marker type for gRPC clients.
///
/// The concrete builder lives in `core/`; callers use
/// [`crate::create_transport_from_config`] instead of direct construction.
#[allow(dead_code)]
pub struct GrpcClientBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_client_builder_is_constructable() {
        let _ = GrpcClientBuilder;
    }
}
