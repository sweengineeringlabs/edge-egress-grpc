//! `GrpcBreakerClient` — re-declared in api/types/ per SEA rule 211.

/// Type alias so the SAF return type is traceable to api/types/.
pub type GrpcBreakerClient<T> = crate::api::breaker_client::GrpcBreakerClient<T>;
