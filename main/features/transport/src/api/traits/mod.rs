//! Primary trait contracts for `swe-edge-egress-grpc-transport`.

#[allow(clippy::module_inception)]
pub mod traits;
pub use traits::{Processor, Validator};
