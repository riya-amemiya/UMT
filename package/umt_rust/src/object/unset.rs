use super::Value;
use std::collections::HashMap;

fn unset_in_object(object: &HashMap<String, Value>, path: &[String]) -> HashMap<String, Value> {
    let mut copy = object.clone();
    let Some((key, rest)) = path.split_first() else {
        return copy;
    };
    if rest.is_empty() {
        copy.remove(key);
        return copy;
    }
    if let Some(value) = copy.get_mut(key) {
        *value = unset_in_value(value, rest);
    }
    copy
}

fn unset_in_value(value: &Value, path: &[String]) -> Value {
    match value {
        Value::Object(map) => Value::Object(unset_in_object(map, path)),
        Value::Array(items) => {
            let mut copy = items.clone();
            let index = super::has::array_index(&path[0]).filter(|index| *index < copy.len());
            if let Some(index) = index {
                if path.len() == 1 {
                    copy.remove(index);
                } else {
                    copy[index] = unset_in_value(&copy[index], &path[1..]);
                }
            }
            Value::Array(copy)
        }
        other => other.clone(),
    }
}

/// Removes the property at a deeply nested path without mutating the input.
///
/// Uses the same path segments as [`super::umt_get`], [`super::umt_set`] and
/// [`super::umt_has_path`]. Mirrors the TypeScript `unset` behavior: objects
/// and arrays along the path are copied, removing an array index removes the
/// element and shifts the following elements, and arrays are traversed by
/// canonical decimal index segments (for example `"0"`). Any other value on
/// the path, or a missing segment, stops the traversal and leaves the rest
/// unchanged. An empty path returns a copy of the object.
///
/// # Prototype pollution warning
///
/// This function does not filter out prototype-polluting keys
/// (`__proto__`, `constructor`, `prototype`). Sanitize user-controlled paths
/// before calling.
///
/// # Arguments
///
/// * `object` - The source object (not mutated).
/// * `path` - The path segments of the property to remove.
///
/// # Returns
///
/// A new object without the property at the path.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_unset, Value};
/// use umt_rust::obj;
///
/// let data = obj!{"a" => obj!{"b" => 1, "c" => 2}};
/// let Value::Object(map) = data else { unreachable!() };
/// let result = umt_unset(&map, &["a".to_string(), "b".to_string()]);
///
/// assert_eq!(Value::Object(result), obj!{"a" => obj!{"c" => 2}});
/// ```
pub fn umt_unset(object: &HashMap<String, Value>, path: &[String]) -> HashMap<String, Value> {
    unset_in_object(object, path)
}

/// Removes the property at a deeply nested path described by a dot-separated
/// string, without mutating the input.
///
/// Convenience wrapper around [`umt_unset`] that splits the path on `.`.
///
/// # Arguments
///
/// * `object` - The source object (not mutated).
/// * `path` - A dot-separated path string.
///
/// # Returns
///
/// A new object without the property at the path.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_unset_path, Value};
/// use umt_rust::obj;
///
/// let data = obj!{"a" => vec![Value::Int(1), Value::Int(2), Value::Int(3)]};
/// let Value::Object(map) = data else { unreachable!() };
/// let result = umt_unset_path(&map, "a.1");
///
/// assert_eq!(result.get("a"), Some(&Value::Array(vec![Value::Int(1), Value::Int(3)])));
/// ```
pub fn umt_unset_path(object: &HashMap<String, Value>, path: &str) -> HashMap<String, Value> {
    umt_unset(object, &super::umt_path_segments(path))
}
