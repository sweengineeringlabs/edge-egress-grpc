//! `GrpcRetryClient` — re-declared in api/types/ per SEA rule 211.

/// Type alias so the SAF return type is traceable to api/types/.
pub type GrpcRetryClient<T> = crate::api::retry_client::GrpcRetryClient<T>;
