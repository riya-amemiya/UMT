//! String type checking functionality
//!
//! This module provides a function to check if a value is a string.

use super::types::DynamicValue;

/// Determines if a DynamicValue is a string
///
/// # Arguments
///
/// * `value` - The dynamic value to check
///
/// # Returns
///
/// `true` if the value is a string, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::{umt_is_string, DynamicValue};
///
/// assert!(umt_is_string(&DynamicValue::String("test".to_string())));
/// assert!(!umt_is_string(&DynamicValue::Integer(123)));
/// ```
#[inline]
pub fn umt_is_string(value: &DynamicValue) -> bool {
    value.is_string()
}

/// Type guard function that always returns true for &str
///
/// This is a compile-time type guard. In Rust, if you have a &str,
/// it's already known to be a string at compile time.
///
/// # Arguments
///
/// * `_value` - The string reference to check
///
/// # Returns
///
/// Always returns `true`
#[inline]
pub fn umt_is_string_ref(_value: &str) -> bool {
    true
}

/// Type guard function that always returns true for String
///
/// # Arguments
///
/// * `_value` - The String to check
///
/// # Returns
///
/// Always returns `true`
#[inline]
pub fn umt_is_string_owned(_value: &String) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_empty_string() {
        assert!(umt_is_string(&DynamicValue::String("".to_string())));
    }

    #[test]
    fn test_regular_strings() {
        assert!(umt_is_string(&DynamicValue::String("test".to_string())));
        assert!(umt_is_string(&DynamicValue::String(
            "hello world".to_string()
        )));
        assert!(umt_is_string(&DynamicValue::String("123".to_string())));
    }

    #[test]
    fn test_string_constructor_result() {
        // In Rust, String::from and to_string both create valid strings
        assert!(umt_is_string(&DynamicValue::String(String::from("test"))));
        assert!(umt_is_string(&DynamicValue::String(123.to_string())));
    }

    #[test]
    fn test_template_literals() {
        let value = "world";
        let formatted = format!("hello {}", value);
        assert!(umt_is_string(&DynamicValue::String(formatted)));
    }

    #[test]
    fn test_numbers_are_not_strings() {
        assert!(!umt_is_string(&DynamicValue::Integer(123)));
        assert!(!umt_is_string(&DynamicValue::Integer(0)));
        assert!(!umt_is_string(&DynamicValue::Integer(-1)));
        assert!(!umt_is_string(&DynamicValue::Float(3.14)));
    }

    #[test]
    fn test_null_is_not_string() {
        assert!(!umt_is_string(&DynamicValue::Null));
    }

    #[test]
    fn test_booleans_are_not_strings() {
        assert!(!umt_is_string(&DynamicValue::Bool(true)));
        assert!(!umt_is_string(&DynamicValue::Bool(false)));
    }

    #[test]
    fn test_objects_and_arrays_are_not_strings() {
        assert!(!umt_is_string(&DynamicValue::Object(HashMap::new())));
        assert!(!umt_is_string(&DynamicValue::Array(vec![])));
    }

    #[test]
    fn test_string_ref() {
        assert!(umt_is_string_ref("test"));
        assert!(umt_is_string_ref(""));
        assert!(umt_is_string_ref("hello world"));
    }

    #[test]
    fn test_string_owned() {
        assert!(umt_is_string_owned(&String::from("test")));
        assert!(umt_is_string_owned(&String::new()));
    }
}
