//! `GrpcRequestBuilder` — builder for [`GrpcRequest`].

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use super::grpc_metadata::GrpcMetadata;
use super::grpc_request::GrpcRequest;

/// Builder for [`GrpcRequest`].
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct GrpcRequestBuilder {
    method:            Option<String>,
    body:              Vec<u8>,
    deadline:          Option<Duration>,
    metadata:          GrpcMetadata,
    cancellation_token: Option<CancellationToken>,
}

#[allow(dead_code)]
impl GrpcRequestBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn method(mut self, v: impl Into<String>) -> Self { self.method = Some(v.into()); self }
    pub fn body(mut self, v: Vec<u8>) -> Self { self.body = v; self }
    pub fn deadline(mut self, v: Duration) -> Self { self.deadline = Some(v); self }
    pub fn metadata(mut self, v: GrpcMetadata) -> Self { self.metadata = v; self }
    pub fn cancellation_token(mut self, v: CancellationToken) -> Self { self.cancellation_token = Some(v); self }

    /// Build the [`GrpcRequest`]. Returns `Err` when method or deadline is unset.
    pub fn build(self) -> Result<GrpcRequest, String> {
        let method   = self.method.ok_or("method required")?;
        let deadline = self.deadline.ok_or("deadline required")?;
        let mut req  = GrpcRequest::new(method, self.body, deadline);
        req.metadata = self.metadata;
        if let Some(token) = self.cancellation_token {
            req = req.with_cancellation(token);
        }
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: build
    /// @covers: method
    #[test]
    fn test_build_valid_request_returns_ok() {
        let req = GrpcRequestBuilder::new()
            .method("svc/Method")
            .deadline(Duration::from_secs(5))
            .build();
        assert!(req.is_ok());
    }

    /// @covers: build
    #[test]
    fn test_build_missing_method_returns_err() {
        assert!(GrpcRequestBuilder::new().deadline(Duration::from_secs(1)).build().is_err());
    }

    /// @covers: body
    #[test]
    fn test_body_setter_stores_bytes() {
        let req = GrpcRequestBuilder::new()
            .method("svc/M")
            .deadline(Duration::from_secs(1))
            .body(vec![1, 2, 3])
            .build()
            .unwrap();
        assert_eq!(req.body, vec![1u8, 2, 3]);
    }

    /// @covers: deadline
    #[test]
    fn test_deadline_setter_is_required_for_build() {
        let req = GrpcRequestBuilder::new()
            .method("svc/M")
            .deadline(Duration::from_secs(10))
            .build()
            .unwrap();
        assert_eq!(req.deadline, Duration::from_secs(10));
    }

    /// @covers: metadata
    #[test]
    fn test_metadata_setter_stores_headers() {
        let mut meta = GrpcMetadata::default();
        meta.headers.insert("x-test".into(), "value".into());
        let req = GrpcRequestBuilder::new()
            .method("svc/M")
            .deadline(Duration::from_secs(1))
            .metadata(meta)
            .build()
            .unwrap();
        assert_eq!(req.metadata.headers.get("x-test").map(String::as_str), Some("value"));
    }

    /// @covers: cancellation_token
    #[test]
    fn test_cancellation_token_setter_attaches_token() {
        use tokio_util::sync::CancellationToken;
        let token = CancellationToken::new();
        let req = GrpcRequestBuilder::new()
            .method("svc/M")
            .deadline(Duration::from_secs(1))
            .cancellation_token(token.clone())
            .build()
            .unwrap();
        assert!(req.cancellation.is_some());
    }
}
