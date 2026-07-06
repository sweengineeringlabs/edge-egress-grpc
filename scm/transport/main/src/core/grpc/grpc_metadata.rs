//! `impl` block for [`GrpcMetadata`]. The type *declaration* lives in `api/`.

use crate::api::GrpcMetadata;

impl GrpcMetadata {
    /// Add a single header entry, returning the modified metadata.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}
