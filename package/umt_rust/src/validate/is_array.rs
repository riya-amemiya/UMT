//! Array type checking functionality
//!
//! This module provides a function to check if a value is an array.

use super::types::DynamicValue;

/// Determines if the value is an array
///
/// # Arguments
///
/// * `value` - The dynamic value to check
///
/// # Returns
///
/// `true` if the value is an array, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::{umt_is_array, DynamicValue};
///
/// assert!(umt_is_array(&DynamicValue::Array(vec![])));
/// assert!(!umt_is_array(&DynamicValue::String("test".to_string())));
/// ```
#[inline]
pub fn umt_is_array(value: &DynamicValue) -> bool {
    value.is_array()
}

/// Determines if a slice is an array (always true for slices)
///
/// # Arguments
///
/// * `_array` - The slice to check
///
/// # Returns
///
/// Always returns `true` since a slice is always an array
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_array_slice;
///
/// assert!(umt_is_array_slice(&[1, 2, 3]));
/// assert!(umt_is_array_slice::<i32>(&[]));
/// ```
#[inline]
pub fn umt_is_array_slice<T>(_array: &[T]) -> bool {
    true
}

/// Determines if a Vec is an array (always true for Vec)
///
/// # Arguments
///
/// * `_array` - The Vec to check
///
/// # Returns
///
/// Always returns `true` since a Vec is always an array
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_array_vec;
///
/// assert!(umt_is_array_vec(&vec![1, 2, 3]));
/// assert!(umt_is_array_vec::<i32>(&vec![]));
/// ```
#[inline]
pub fn umt_is_array_vec<T>(_array: &Vec<T>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_is_array_returns_true_for_arrays() {
        assert!(umt_is_array(&DynamicValue::Array(vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3)
        ])));
        assert!(umt_is_array(&DynamicValue::Array(vec![
            DynamicValue::String("a".to_string()),
            DynamicValue::String("b".to_string()),
            DynamicValue::String("c".to_string())
        ])));
        assert!(umt_is_array(&DynamicValue::Array(vec![
            DynamicValue::Bool(true),
            DynamicValue::Bool(false)
        ])));
        assert!(umt_is_array(&DynamicValue::Array(vec![])));
    }

    #[test]
    fn test_is_array_returns_false_for_non_arrays() {
        assert!(!umt_is_array(&DynamicValue::Integer(1)));
        assert!(!umt_is_array(&DynamicValue::String("hello".to_string())));
        assert!(!umt_is_array(&DynamicValue::Bool(true)));
        assert!(!umt_is_array(&DynamicValue::Object(HashMap::new())));
        assert!(!umt_is_array(&DynamicValue::Null));
    }

    #[test]
    fn test_is_array_slice() {
        assert!(umt_is_array_slice(&[1, 2, 3]));
        assert!(umt_is_array_slice(&["a", "b", "c"]));
        assert!(umt_is_array_slice(&[true, false]));
        assert!(umt_is_array_slice::<i32>(&[]));
    }

    #[test]
    fn test_is_array_vec() {
        assert!(umt_is_array_vec(&vec![1, 2, 3]));
        assert!(umt_is_array_vec(&vec!["a", "b", "c"]));
        assert!(umt_is_array_vec(&vec![true, false]));
        assert!(umt_is_array_vec::<i32>(&vec![]));
    }
}
