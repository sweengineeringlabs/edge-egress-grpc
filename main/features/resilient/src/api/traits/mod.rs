//! SEA interface contract — primary traits for `swe-edge-egress-grpc-resilient`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |

pub mod processor;
pub use processor::Processor;
