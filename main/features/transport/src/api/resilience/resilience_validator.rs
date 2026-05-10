//! Interface counterpart for `core/resilience/resilience_validator.rs`.
//!
//! The `Validator` trait (defined in `api/traits.rs`) is the interface contract
//! implemented on `ResilienceConfig`.

#[cfg(test)]
mod tests {
    use crate::api::traits::Validator;

    #[test]
    fn test_validator_re_export_is_object_safe() {
        fn _assert(_: &dyn Validator) {}
    }
}
