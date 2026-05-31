//! gRPC request/response metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata for a gRPC request/response (headers).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrpcMetadata {
    pub headers: HashMap<String, String>,
}

impl GrpcMetadata {
    /// Add a single header entry, returning the modified metadata.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}
