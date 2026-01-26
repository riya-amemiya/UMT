//! String validation module for UUID strings
//!
//! Provides validation functionality for checking if a string is a valid UUID.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a string is a valid UUID
///
/// # Arguments
///
/// * `versions` - Array of supported UUID versions (default: [4])
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for UUID strings
pub fn umt_uuid(versions: Option<Vec<u8>>, message: Option<&str>) -> ValidateReturnType<String> {
    let versions = versions.unwrap_or_else(|| vec![4]);
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| is_valid_uuid(value, &versions),
        message.map(String::from),
    )
}

fn is_valid_uuid(value: &str, versions: &[u8]) -> bool {
    let hex_only: String = value.chars().filter(|c| *c != '-').collect();
    if hex_only.len() != 32 {
        return false;
    }
    if !hex_only.chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    // Check version (13th hex char)
    let version_char = hex_only.as_bytes()[12];
    let version_num = if version_char.is_ascii_digit() {
        version_char - b'0'
    } else {
        return false;
    };
    if !versions.contains(&version_num) {
        return false;
    }
    // Check variant (17th hex char should be 8, 9, a, or b)
    let variant_char = hex_only.as_bytes()[16].to_ascii_lowercase();
    matches!(variant_char, b'8' | b'9' | b'a' | b'b')
}
