//! Gateway layer — crate entry boundary.
//!
//! Exports the curated public surface via the SAF facade.

pub(crate) mod input;
pub(crate) mod output;

pub use crate::saf::*;
