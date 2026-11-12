use super::Value;
use std::collections::HashMap;

/// Sets a value at a deeply nested path from its current value, mutating the
/// object.
///
/// Reads the current value like [`super::umt_get`] and writes the result like
/// [`super::umt_set`]. Mirrors the TypeScript `update` behavior: the updater
/// receives `None` (JavaScript `undefined`) when the path is missing, and
/// intermediate segments that are missing or not objects are replaced with
/// new objects. An empty path leaves the object unchanged and does not call
/// the updater.
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
/// * `path` - The path segments to update.
/// * `updater` - Computes the new value from the current value.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_update, Value};
/// use umt_rust::obj;
///
/// let mut map = obj!{"a" => obj!{"b" => 1}}.into_object().unwrap();
/// umt_update(&mut map, &["a".to_string(), "b".to_string()], |value| match value {
///     Some(Value::Int(n)) => Value::Int(n + 1),
///     _ => Value::Int(0),
/// });
///
/// assert_eq!(Value::Object(map), obj!{"a" => obj!{"b" => 2}});
/// ```
pub fn umt_update<F>(object: &mut HashMap<String, Value>, path: &[String], updater: F)
where
    F: FnOnce(Option<Value>) -> Value,
{
    let Some((first, rest)) = path.split_first() else {
        return;
    };
    let current = object
        .get(first)
        .and_then(|value| super::umt_get(value, rest, None));
    super::umt_set(object, path, updater(current));
}

/// Sets a value at a deeply nested path described by a dot-separated string
/// from its current value, mutating the object.
///
/// Convenience wrapper around [`umt_update`] that splits the path on `.`.
///
/// # Arguments
///
/// * `object` - The target object (mutated in place).
/// * `path` - A dot-separated path string.
/// * `updater` - Computes the new value from the current value.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_update_path, Value};
/// use std::collections::HashMap;
///
/// let mut map: HashMap<String, Value> = HashMap::new();
/// umt_update_path(&mut map, "a.b", |value| value.unwrap_or(Value::Int(0)));
///
/// let a = map.get("a").and_then(Value::as_object).unwrap();
/// assert_eq!(a.get("b"), Some(&Value::Int(0)));
/// ```
pub fn umt_update_path<F>(object: &mut HashMap<String, Value>, path: &str, updater: F)
where
    F: FnOnce(Option<Value>) -> Value,
{
    umt_update(object, &super::umt_path_segments(path), updater);
}
