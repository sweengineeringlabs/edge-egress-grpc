//! Error type for the gRPC breaker decorator.

/// Errors raised while loading config or building the breaker.
///
/// Runtime errors from the wrapped [`GrpcEgress`] flow through
/// as the inner crate's `GrpcEgressError`; they are not
/// re-wrapped here.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Config TOML didn't parse as the expected schema.
    #[error("swe_edge_egress_grpc_breaker: config parse failed — {0}")]
    ParseFailed(String),

    /// A configured value is invalid (e.g. zero failure_threshold).
    #[error("swe_edge_egress_grpc_breaker: invalid config — {0}")]
    InvalidConfig(String),
}
