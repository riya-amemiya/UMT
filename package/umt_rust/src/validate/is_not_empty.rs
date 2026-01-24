//! Empty object checking functionality
//!
//! This module provides a function to check if an object is not empty.

use super::types::DynamicValue;
use std::collections::HashMap;

/// Checks if a DynamicValue object/array is not empty
///
/// # Arguments
///
/// * `value` - The DynamicValue to check
///
/// # Returns
///
/// `true` if the object/array has at least one element, `false` if empty
///
/// # Examples
///
/// ```
/// use umt_rust::validate::{umt_is_not_empty, DynamicValue};
/// use std::collections::HashMap;
///
/// let mut obj = HashMap::new();
/// obj.insert("a".to_string(), DynamicValue::Integer(1));
/// assert!(umt_is_not_empty(&DynamicValue::Object(obj)));
///
/// let empty_obj = HashMap::new();
/// assert!(!umt_is_not_empty(&DynamicValue::Object(empty_obj)));
/// ```
#[inline]
pub fn umt_is_not_empty(value: &DynamicValue) -> bool {
    match value {
        DynamicValue::Object(obj) => !obj.is_empty(),
        DynamicValue::Array(arr) => !arr.is_empty(),
        DynamicValue::String(s) => !s.is_empty(),
        _ => false,
    }
}

/// Checks if a HashMap is not empty
///
/// # Arguments
///
/// * `object` - The HashMap to check
///
/// # Returns
///
/// `true` if the object has at least one key, `false` if empty
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_not_empty_hashmap;
/// use std::collections::HashMap;
///
/// let mut obj = HashMap::new();
/// obj.insert("a", 1);
/// assert!(umt_is_not_empty_hashmap(&obj));
///
/// let empty: HashMap<&str, i32> = HashMap::new();
/// assert!(!umt_is_not_empty_hashmap(&empty));
/// ```
#[inline]
pub fn umt_is_not_empty_hashmap<K, V>(object: &HashMap<K, V>) -> bool {
    !object.is_empty()
}

/// Checks if a slice is not empty
///
/// # Arguments
///
/// * `array` - The slice to check
///
/// # Returns
///
/// `true` if the slice has at least one element, `false` if empty
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_not_empty_slice;
///
/// assert!(umt_is_not_empty_slice(&[1, 2, 3]));
/// assert!(!umt_is_not_empty_slice::<i32>(&[]));
/// ```
#[inline]
pub fn umt_is_not_empty_slice<T>(array: &[T]) -> bool {
    !array.is_empty()
}

/// Checks if a Vec is not empty
///
/// # Arguments
///
/// * `array` - The Vec to check
///
/// # Returns
///
/// `true` if the Vec has at least one element, `false` if empty
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_not_empty_vec;
///
/// assert!(umt_is_not_empty_vec(&vec![1, 2, 3]));
/// assert!(!umt_is_not_empty_vec::<i32>(&vec![]));
/// ```
#[inline]
pub fn umt_is_not_empty_vec<T>(array: &Vec<T>) -> bool {
    !array.is_empty()
}

/// Checks if a string is not empty
///
/// # Arguments
///
/// * `value` - The string to check
///
/// # Returns
///
/// `true` if the string has at least one character, `false` if empty
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_not_empty_str;
///
/// assert!(umt_is_not_empty_str("hello"));
/// assert!(!umt_is_not_empty_str(""));
/// ```
#[inline]
pub fn umt_is_not_empty_str(value: &str) -> bool {
    !value.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_true_for_non_empty_object() {
        let mut obj = HashMap::new();
        obj.insert("a".to_string(), DynamicValue::Integer(1));
        assert!(umt_is_not_empty(&DynamicValue::Object(obj)));

        let mut obj2 = HashMap::new();
        obj2.insert("a".to_string(), DynamicValue::Integer(1));
        obj2.insert("b".to_string(), DynamicValue::Integer(2));
        assert!(umt_is_not_empty(&DynamicValue::Object(obj2)));

        let mut nested = HashMap::new();
        let mut inner = HashMap::new();
        inner.insert("b".to_string(), DynamicValue::Integer(2));
        nested.insert("a".to_string(), DynamicValue::Object(inner));
        assert!(umt_is_not_empty(&DynamicValue::Object(nested)));
    }

    #[test]
    fn test_returns_false_for_empty_object() {
        let obj: HashMap<String, DynamicValue> = HashMap::new();
        assert!(!umt_is_not_empty(&DynamicValue::Object(obj)));
    }

    #[test]
    fn test_returns_true_for_non_empty_array() {
        let arr = vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3),
        ];
        assert!(umt_is_not_empty(&DynamicValue::Array(arr)));

        let arr_str = vec![
            DynamicValue::String("a".to_string()),
            DynamicValue::String("b".to_string()),
        ];
        assert!(umt_is_not_empty(&DynamicValue::Array(arr_str)));

        let arr_bool = vec![DynamicValue::Bool(true), DynamicValue::Bool(false)];
        assert!(umt_is_not_empty(&DynamicValue::Array(arr_bool)));

        let mut obj1 = HashMap::new();
        obj1.insert("a".to_string(), DynamicValue::Integer(1));
        let mut obj2 = HashMap::new();
        obj2.insert("b".to_string(), DynamicValue::Integer(2));
        let arr_obj = vec![DynamicValue::Object(obj1), DynamicValue::Object(obj2)];
        assert!(umt_is_not_empty(&DynamicValue::Array(arr_obj)));
    }

    #[test]
    fn test_returns_false_for_empty_array() {
        let arr: Vec<DynamicValue> = vec![];
        assert!(!umt_is_not_empty(&DynamicValue::Array(arr)));
    }

    #[test]
    fn test_returns_false_for_non_container_types() {
        // Note: Map and Set behavior in TS tests - in Rust, we use different types
        // For DynamicValue, non-container types return false
        assert!(!umt_is_not_empty(&DynamicValue::Null));
        assert!(!umt_is_not_empty(&DynamicValue::Bool(true)));
        assert!(!umt_is_not_empty(&DynamicValue::Integer(42)));
    }

    #[test]
    fn test_hashmap_not_empty() {
        let mut obj = HashMap::new();
        obj.insert("a", 1);
        obj.insert("b", 2);
        assert!(umt_is_not_empty_hashmap(&obj));

        let empty: HashMap<&str, i32> = HashMap::new();
        assert!(!umt_is_not_empty_hashmap(&empty));
    }

    #[test]
    fn test_slice_not_empty() {
        assert!(umt_is_not_empty_slice(&[1, 2, 3]));
        assert!(umt_is_not_empty_slice(&["a", "b", "c"]));
        assert!(umt_is_not_empty_slice(&[true, false]));
        assert!(!umt_is_not_empty_slice::<i32>(&[]));
    }

    #[test]
    fn test_vec_not_empty() {
        assert!(umt_is_not_empty_vec(&vec![1, 2, 3]));
        assert!(!umt_is_not_empty_vec::<i32>(&vec![]));
    }

    #[test]
    fn test_string_not_empty() {
        assert!(umt_is_not_empty_str("hello"));
        assert!(umt_is_not_empty_str(" "));
        assert!(!umt_is_not_empty_str(""));
    }

    #[test]
    fn test_dynamic_value_string() {
        assert!(umt_is_not_empty(&DynamicValue::String("hello".to_string())));
        assert!(!umt_is_not_empty(&DynamicValue::String("".to_string())));
    }
}
