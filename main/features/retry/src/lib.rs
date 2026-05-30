//! `swe_edge_egress_grpc_retry` — decorator that wraps a
//! [`GrpcEgress`](swe_edge_egress_grpc::GrpcEgress) with
//! exponential-backoff-with-jitter retry on gRPC-canonical
//! retryable status codes.
//!
//! ## Retry policy
//!
//! Retries on `Unavailable` and `ResourceExhausted` (both are
//! transient on the gRPC side and have a real chance of
//! succeeding on a fresh attempt).  Never retries
//! `Unauthenticated` / `PermissionDenied` (those don't get better
//! by trying again — they need a credential refresh upstream).
//! Never retries `DeadlineExceeded` (the caller's deadline is
//! the retry budget; eating the deadline on a re-issue is
//! pointless) or `Internal` (server bug — amplifying it just
//! amplifies the bug).
//!
//! ## Deadline budget
//!
//! Total retries are bounded by the caller's deadline carried
//! on [`GrpcRequest::deadline`](swe_edge_egress_grpc::GrpcRequest).
//! Even if the configured `max_attempts` allows more tries, the
//! loop stops when the elapsed time + next backoff would
//! overrun the deadline.
//!
//! ## Composition
//!
//! Wrap any [`GrpcEgress`] implementor:
//!
//! ```ignore
//! use swe_edge_egress_grpc_retry::{wrap_retry, GrpcRetryConfig};
//!
//! let inner   = my_grpc_client();
//! let retried = wrap_retry(inner, GrpcRetryConfig::default());
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]

mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
