//! Error returned when a channel configuration violates a fail-closed invariant.

/// Error returned by [`crate::saf::create_transport_from_config`] when the
/// channel configuration violates a fail-closed invariant.
#[derive(Debug, thiserror::Error)]
pub enum GrpcChannelConfigError {
    /// `tls_required` is set but the endpoint scheme is plaintext.
    #[error("plaintext endpoint '{0}' rejected — tls_required is set; use .allow_plaintext() to opt out")]
    PlaintextRejected(String),
    /// A resilience config field is invalid (e.g. `max_attempts = 0`).
    #[error("invalid resilience config: {0}")]
    Config(String),
}
