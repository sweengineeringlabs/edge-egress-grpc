//! `TonicGrpcClient` — re-declared in api/types/ per SEA rule 211.

/// Type alias so the SAF return type is traceable to api/types/.
pub type TonicGrpcClient = crate::api::client::tonic_grpc_client::TonicGrpcClient;
