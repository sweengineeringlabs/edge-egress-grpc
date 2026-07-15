//! Error type for the gRPC retry decorator.

/// Errors raised while loading config or building the retry layer.
///
/// Runtime errors from the wrapped [`GrpcEgress`] flow through
/// as the inner crate's `GrpcEgressError`; they are not
/// re-wrapped here.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Config TOML didn't parse as the expected schema.
    /// Wraps the underlying `toml::de::Error` message, which
    /// names the missing or unknown field when that's the cause.
    #[error("edge_transport_grpc_egress_retry: config parse failed — {0}")]
    ParseFailed(String),

    /// A configured numeric value is outside its valid range
    /// (e.g. `backoff_multiplier <= 0.0`).
    #[error("edge_transport_grpc_egress_retry: invalid config — {0}")]
    InvalidConfig(String),
}
