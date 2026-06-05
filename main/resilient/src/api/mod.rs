//! API layer — error types, traits, and interface contracts.

pub mod error;
pub use error::*;

pub(crate) mod factory;
pub(crate) mod processor;
pub mod traits;

pub mod types;
pub use types::*;
