//! Gateway layer — public surface for the gRPC outbound transport crate.
pub(crate) mod input;
pub(crate) mod output;

pub use output::*;
