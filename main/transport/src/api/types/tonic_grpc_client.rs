//! `TonicGrpcClient` — re-declared in api/types/ per SEA rule 211.

/// Type alias so the SAF return type is traceable to api/types/.
pub type TonicGrpcClient = crate::api::types::client::tonic_grpc_client::TonicGrpcClient;
