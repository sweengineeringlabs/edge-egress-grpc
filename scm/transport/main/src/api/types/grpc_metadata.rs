//! gRPC request/response metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata for a gRPC request/response (headers).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrpcMetadata {
    /// Key-value pairs to send as gRPC metadata headers.
    pub headers: HashMap<String, String>,
}
