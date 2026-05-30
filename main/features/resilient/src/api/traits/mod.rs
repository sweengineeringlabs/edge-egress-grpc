//! Primary trait declarations for `swe-edge-egress-grpc-resilient`.
//!
//! | Trait | Contract |
//! |---|---|
//! | [`Processor`] | Primary processing trait for this service_type = "processor" crate |
//! | [`Validator`] | Configuration validation contract |

pub mod processor;
pub mod validator;

pub use processor::Processor;
pub use validator::Validator;
