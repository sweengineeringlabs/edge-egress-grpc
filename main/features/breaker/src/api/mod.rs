//! API layer — public schema + trait declarations + public types.
//!
//! Per SEA rule 160, public type *declarations* live here.  Impl
//! blocks live in `core/`.

pub(crate) mod breaker;
pub mod error;
pub mod traits;

pub mod types;
pub use types::*;

pub use breaker::BreakerState;
pub use breaker::Error;
pub use breaker::GrpcBreakerClient;
pub use breaker::GrpcBreakerConfig;
