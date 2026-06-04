use super::Value;

/// Reads a deeply nested property by a path of segments.
///
/// Returns the default value if any segment is missing or if traversal
/// passes through a null value. Mirrors the TypeScript `get` behavior where
/// reaching `null`/`undefined` mid-path, or resolving to `undefined`, falls
/// back to the default value. A value that resolves to `Value::Null` is
/// returned as-is (only `undefined`, i.e. a missing key, triggers the
/// default).
///
/// # Arguments
///
/// * `object` - The source value to traverse.
/// * `path` - The path segments to follow.
/// * `default_value` - Returned when the path is missing or resolves to undefined.
///
/// # Returns
///
/// The resolved value, or `default_value` when the path is missing.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_get, Value};
/// use umt_rust::obj;
///
/// let data = obj!{"a" => obj!{"b" => obj!{"c" => 1}}};
/// assert_eq!(
///     umt_get(&data, &["a".to_string(), "b".to_string(), "c".to_string()], None),
///     Some(Value::Int(1))
/// );
/// assert_eq!(
///     umt_get(&data, &["a".to_string(), "x".to_string()], Some(Value::Int(0))),
///     Some(Value::Int(0))
/// );
/// ```
pub fn umt_get(object: &Value, path: &[String], default_value: Option<Value>) -> Option<Value> {
    // `None` models a JavaScript `undefined` during traversal.
    let mut current: Option<&Value> = Some(object);

    for key in path {
        match current {
            // `null` or `undefined` short-circuits to the default value.
            None | Some(Value::Null) => return default_value,
            Some(Value::Object(map)) => {
                current = map.get(key);
            }
            // Indexing into a non-object primitive yields `undefined`.
            Some(_) => {
                current = None;
            }
        }
    }

    match current {
        // A missing/undefined resolution falls back to the default.
        None => default_value,
        Some(value) => Some(value.clone()),
    }
}

/// Reads a deeply nested property by a dot-separated path string.
///
/// Convenience wrapper around [`umt_get`] that splits the path on `.`.
///
/// # Arguments
///
/// * `object` - The source value to traverse.
/// * `path` - A dot-separated path string.
/// * `default_value` - Returned when the path is missing or resolves to undefined.
///
/// # Returns
///
/// The resolved value, or `default_value` when the path is missing.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_get_path, Value};
/// use umt_rust::obj;
///
/// let data = obj!{"a" => obj!{"b" => obj!{"c" => 42}}};
/// assert_eq!(umt_get_path(&data, "a.b.c", None), Some(Value::Int(42)));
/// assert_eq!(
///     umt_get_path(&data, "a.x", Some(Value::String("fallback".to_string()))),
///     Some(Value::String("fallback".to_string()))
/// );
/// ```
pub fn umt_get_path(object: &Value, path: &str, default_value: Option<Value>) -> Option<Value> {
    let segments = super::umt_path_segments(path);
    umt_get(object, &segments, default_value)
}
