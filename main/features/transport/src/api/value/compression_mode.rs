//! Compression negotiation modes for gRPC channels.

use serde::{Deserialize, Serialize};

/// Wire-level compression scheme for gRPC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CompressionMode {
    /// No compression.
    #[default]
    None,
    /// gzip compression.
    Gzip,
    /// zstandard compression.
    Zstd,
}

impl CompressionMode {
    /// `grpc-encoding` header value, or `None` for identity-only.
    pub fn header_value(self) -> Option<&'static str> {
        match self {
            CompressionMode::None => None,
            CompressionMode::Gzip => Some("gzip"),
            CompressionMode::Zstd => Some("zstd"),
        }
    }
}
