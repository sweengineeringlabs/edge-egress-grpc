//! Interface counterpart for `core/resilience/resilience_validator.rs`.
//!
//! The `Validator` trait (defined in `api/traits.rs`) is the interface contract
//! implemented on `ResilienceConfig`.

pub use crate::api::traits::Validator;
pub use crate::api::value_object::ResilienceConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_re_export_is_object_safe() {
        fn _assert(_: &dyn Validator) {}
    }
}
