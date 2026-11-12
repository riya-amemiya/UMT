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
///
/// assert!(umt_has(&obj, "a.b"));
/// assert!(!umt_has(&obj, "a.c"));
/// ```
pub fn umt_has(object: &HashMap<String, Value>, path: &str) -> bool {
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
///
/// Arrays are traversed by canonical decimal index segments (for example
/// `"0"`), mirroring own index keys of JavaScript arrays.
pub fn umt_has_path(object: &HashMap<String, Value>, path: &[&str]) -> bool {
    let Some((first, rest)) = path.split_first() else {
        return true;
    };

    let Some(mut current) = object.get(*first) else {
        return false;
    };

    for key in rest {
        let next = match current {
            Value::Object(inner) => inner.get(*key),
            Value::Array(items) => array_index(key).and_then(|index| items.get(index)),
            _ => None,
        };

        match next {
            Some(value) => current = value,
            None => return false,
        }
    }

    true
}

/// Parses a path segment as a canonical array index (`"0"`, `"12"`, but not
/// `"01"`, `"+1"` or `""`).
pub(crate) fn array_index(key: &str) -> Option<usize> {
    let is_canonical =
        key == "0" || (!key.starts_with('0') && key.bytes().all(|byte| byte.is_ascii_digit()));
    if key.is_empty() || !is_canonical {
        return None;
    }
    key.parse().ok()
}

/// Helper function to convert a dot-separated path to parts.
fn path_to_parts(path: &str) -> Vec<&str> {
    if path.is_empty() {
        return vec![""];
    }
    path.split('.').collect()
}
