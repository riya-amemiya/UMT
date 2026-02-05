use super::Value;
use std::collections::HashMap;

/// Determines if an object has a specified path.
///
/// # Arguments
///
/// * `object` - The object to check
/// * `path` - The path to check (dot-separated string or vector of keys)
///
/// # Returns
///
/// Returns true if path exists, false otherwise.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_has, Value};
/// use std::collections::HashMap;
///
/// let mut inner = HashMap::new();
/// inner.insert("b".to_string(), Value::Int(1));
/// let mut obj = HashMap::new();
/// obj.insert("a".to_string(), Value::Object(inner));
/// let value = Value::Object(obj);
///
/// assert!(umt_has(&value, "a.b"));
/// assert!(!umt_has(&value, "a.c"));
/// ```
pub fn umt_has(object: &Value, path: &str) -> bool {
    umt_has_path(object, &path_to_parts(path))
}

/// Determines if an object has a specified path (array form).
///
/// # Arguments
///
/// * `object` - The object to check
/// * `path` - The path as a slice of string keys
///
/// # Returns
///
/// Returns true if path exists, false otherwise.
pub fn umt_has_path(object: &Value, path: &[&str]) -> bool {
    if path.is_empty() {
        return true;
    }

    let mut current = object;

    for (i, key) in path.iter().enumerate() {
        if key.is_empty() {
            return false;
        }

        match current {
            Value::Object(map) => {
                match map.get(*key) {
                    Some(value) => {
                        if i == path.len() - 1 {
                            return true;
                        }
                        current = value;
                    }
                    None => return false,
                }
            }
            _ => return false,
        }
    }

    true
}

/// Helper function to convert a dot-separated path to parts.
fn path_to_parts(path: &str) -> Vec<&str> {
    if path.is_empty() {
        return vec![""];
    }
    path.split('.').collect()
}
