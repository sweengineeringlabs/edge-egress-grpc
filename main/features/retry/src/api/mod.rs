//! API layer — public schema + trait declarations + public types.
//!
//! Per SEA rule 160, public type *declarations* live here.  Impl
//! blocks live in `core/`.

pub mod error;
pub mod traits;
pub mod types;

pub use error::Error;
pub use types::*;
pub(crate) mod backoff;
pub(crate) mod processor;
pub(crate) mod retry;
