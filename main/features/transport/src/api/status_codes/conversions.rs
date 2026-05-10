//! `Conversions` — interface counterpart for `core/status_codes/conversions.rs`.

/// Marker type identifying this as the status code conversion interface module.
#[allow(dead_code)]
pub struct Conversions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions_marker_is_constructable() {
        let _ = Conversions;
    }
}
