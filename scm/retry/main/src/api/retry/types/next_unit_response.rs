//! Response for [`crate::api::JitterRng::next_unit`].

/// Output of [`crate::api::JitterRng::next_unit`] — a uniform sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NextUnitResponse {
    /// Uniform sample in `[0.0, 1.0)`.
    pub value: f64,
}
