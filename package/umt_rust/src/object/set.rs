use super::Value;
use std::collections::HashMap;

/// Sets a value at a deeply nested path on an object, mutating it.
///
/// Creates intermediate objects as needed. Mirrors the TypeScript `set`
/// behavior: an empty path leaves the object unchanged, and any intermediate
/// segment that is missing or not an object is replaced with a fresh object.
///
/// # Prototype pollution warning
///
/// This function does not filter out prototype-polluting keys
/// (`__proto__`, `constructor`, `prototype`). Sanitize user-controlled paths
/// before calling.
///
/// # Arguments
///
/// * `object` - The target object (mutated in place).
/// * `path` - The path segments to assign at.
/// * `value` - The value to assign at the resolved path.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_set, Value};
/// use std::collections::HashMap;
///
/// let mut target: HashMap<String, Value> = HashMap::new();
/// umt_set(&mut target, &["a".to_string(), "b".to_string(), "c".to_string()], Value::Int(1));
///
/// if let Some(Value::Object(a)) = target.get("a") {
///     if let Some(Value::Object(b)) = a.get("b") {
///         assert_eq!(b.get("c"), Some(&Value::Int(1)));
///     }
/// }
/// ```
pub fn umt_set(object: &mut HashMap<String, Value>, path: &[String], value: Value) {
    if path.is_empty() {
        return;
    }

    let mut current = object;

    for key in &path[..path.len() - 1] {
        // Replace missing or non-object intermediates with a fresh object.
        let needs_object = !matches!(current.get(key), Some(Value::Object(_)));
        if needs_object {
            current.insert(key.clone(), Value::Object(HashMap::new()));
        }
        current = match current.get_mut(key) {
            Some(Value::Object(map)) => map,
            _ => unreachable!("intermediate was just ensured to be an object"),
        };
    }

    let last = &path[path.len() - 1];
    current.insert(last.clone(), value);
}

/// Sets a value at a deeply nested path described by a dot-separated string.
///
/// Convenience wrapper around [`umt_set`] that splits the path on `.`.
///
/// # Arguments
///
/// * `object` - The target object (mutated in place).
/// * `path` - A dot-separated path string.
/// * `value` - The value to assign at the resolved path.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_set_path, Value};
/// use std::collections::HashMap;
///
/// let mut target: HashMap<String, Value> = HashMap::new();
/// umt_set_path(&mut target, "a", Value::Int(1));
/// assert_eq!(target.get("a"), Some(&Value::Int(1)));
/// ```
pub fn umt_set_path(object: &mut HashMap<String, Value>, path: &str, value: Value) {
    let segments = super::umt_path_segments(path);
    umt_set(object, &segments, value);
}
