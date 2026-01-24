//! Dictionary object type checking functionality
//!
//! This module provides a function to check if a value is a dictionary-type object.

use super::types::DynamicValue;
use std::collections::HashMap;

/// Determines if the value is a dictionary-type object
///
/// A dictionary object is an object that is not null and not an array.
///
/// # Arguments
///
/// * `value` - The dynamic value to check
///
/// # Returns
///
/// `true` if the value is a dictionary object, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::{umt_is_dictionary_object, DynamicValue};
/// use std::collections::HashMap;
///
/// let mut obj = HashMap::new();
/// obj.insert("key".to_string(), DynamicValue::Integer(42));
///
/// assert!(umt_is_dictionary_object(&DynamicValue::Object(obj)));
/// assert!(!umt_is_dictionary_object(&DynamicValue::Array(vec![])));
/// assert!(!umt_is_dictionary_object(&DynamicValue::Null));
/// ```
#[inline]
pub fn umt_is_dictionary_object(value: &DynamicValue) -> bool {
    matches!(value, DynamicValue::Object(_))
}

/// Determines if a HashMap is a dictionary object (always true)
///
/// # Arguments
///
/// * `_object` - The HashMap to check
///
/// # Returns
///
/// Always returns `true` since a HashMap is always a dictionary object
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_dictionary_object_hashmap;
/// use std::collections::HashMap;
///
/// let map: HashMap<String, i32> = HashMap::new();
/// assert!(umt_is_dictionary_object_hashmap(&map));
/// ```
#[inline]
pub fn umt_is_dictionary_object_hashmap<K, V>(_object: &HashMap<K, V>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_true_for_empty_object() {
        let obj = HashMap::new();
        assert!(umt_is_dictionary_object(&DynamicValue::Object(obj)));
    }

    #[test]
    fn test_returns_true_for_object_with_string_properties() {
        let mut obj = HashMap::new();
        obj.insert("foo".to_string(), DynamicValue::String("bar".to_string()));
        obj.insert("baz".to_string(), DynamicValue::String("qux".to_string()));
        assert!(umt_is_dictionary_object(&DynamicValue::Object(obj)));
    }

    #[test]
    fn test_returns_true_for_object_with_number_properties() {
        let mut obj = HashMap::new();
        obj.insert("foo".to_string(), DynamicValue::Integer(1));
        obj.insert("bar".to_string(), DynamicValue::Integer(2));
        assert!(umt_is_dictionary_object(&DynamicValue::Object(obj)));
    }

    #[test]
    fn test_returns_true_for_object_with_boolean_properties() {
        let mut obj = HashMap::new();
        obj.insert("foo".to_string(), DynamicValue::Bool(true));
        obj.insert("bar".to_string(), DynamicValue::Bool(false));
        assert!(umt_is_dictionary_object(&DynamicValue::Object(obj)));
    }

    #[test]
    fn test_returns_true_for_object_with_mixed_properties() {
        let mut obj = HashMap::new();
        obj.insert("foo".to_string(), DynamicValue::String("bar".to_string()));
        obj.insert("baz".to_string(), DynamicValue::Integer(1));
        obj.insert("qux".to_string(), DynamicValue::Bool(true));
        assert!(umt_is_dictionary_object(&DynamicValue::Object(obj)));
    }

    #[test]
    fn test_returns_false_for_array() {
        let arr = vec![
            DynamicValue::Integer(1),
            DynamicValue::Integer(2),
            DynamicValue::Integer(3),
        ];
        assert!(!umt_is_dictionary_object(&DynamicValue::Array(arr)));
    }

    #[test]
    fn test_returns_false_for_string() {
        assert!(!umt_is_dictionary_object(&DynamicValue::String(
            "foo".to_string()
        )));
    }

    #[test]
    fn test_returns_false_for_number() {
        assert!(!umt_is_dictionary_object(&DynamicValue::Integer(42)));
        assert!(!umt_is_dictionary_object(&DynamicValue::Float(3.14)));
    }

    #[test]
    fn test_returns_false_for_boolean() {
        assert!(!umt_is_dictionary_object(&DynamicValue::Bool(true)));
        assert!(!umt_is_dictionary_object(&DynamicValue::Bool(false)));
    }

    #[test]
    fn test_returns_false_for_null() {
        assert!(!umt_is_dictionary_object(&DynamicValue::Null));
    }

    #[test]
    fn test_hashmap_check() {
        let map: HashMap<String, i32> = HashMap::new();
        assert!(umt_is_dictionary_object_hashmap(&map));

        let mut map2 = HashMap::new();
        map2.insert("key".to_string(), 42);
        assert!(umt_is_dictionary_object_hashmap(&map2));
    }
}
