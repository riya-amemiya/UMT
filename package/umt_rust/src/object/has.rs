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
pub fn umt_has_path(object: &HashMap<String, Value>, path: &[&str]) -> bool {
    if path.is_empty() {
        return true;
    }

    let mut current: &HashMap<String, Value> = object;

    for (i, key) in path.iter().enumerate() {
        if key.is_empty() {
            return false;
        }

        match current.get(*key) {
            Some(value) => {
                if i == path.len() - 1 {
                    return true;
                }
                match value {
                    Value::Object(inner) => {
                        current = inner;
                    }
                    _ => return false,
                }
            }
            None => return false,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_nested_object() -> HashMap<String, Value> {
        let mut inner = HashMap::new();
        inner.insert("b".to_string(), Value::Int(1));

        let mut obj = HashMap::new();
        obj.insert("a".to_string(), Value::Object(inner));
        obj
    }

    #[test]
    fn test_has_existing_nested_path_string() {
        let obj = create_nested_object();
        assert!(umt_has(&obj, "a.b"));
    }

    #[test]
    fn test_has_existing_nested_path_array() {
        let obj = create_nested_object();
        assert!(umt_has_path(&obj, &["a", "b"]));
    }

    #[test]
    fn test_has_non_existing_nested_path_string() {
        let obj = create_nested_object();
        assert!(!umt_has(&obj, "a.c"));
    }

    #[test]
    fn test_has_non_existing_nested_path_array() {
        let obj = create_nested_object();
        assert!(!umt_has_path(&obj, &["a", "c"]));
    }

    #[test]
    fn test_has_non_existing_top_level_path() {
        let obj = create_nested_object();
        assert!(!umt_has(&obj, "b"));
    }

    #[test]
    fn test_has_empty_path_string() {
        let obj = create_nested_object();
        assert!(!umt_has(&obj, ""));
    }

    #[test]
    fn test_has_empty_path_array() {
        let obj = create_nested_object();
        assert!(umt_has_path(&obj, &[]));
    }

    #[test]
    fn test_has_empty_object() {
        let obj: HashMap<String, Value> = HashMap::new();
        assert!(!umt_has(&obj, "a.b"));
    }
}
