use super::Value;
use super::remove_prototype::DANGEROUS_KEYS;
use std::collections::HashMap;

/// Creates a new object with prototype-polluting properties removed
/// recursively.
///
/// The keys `__proto__`, `constructor` and `prototype` are excluded from
/// nested objects and from objects nested inside arrays. Arrays and plain
/// objects are deep-cloned; other values pass through by clone. Mirrors the
/// TypeScript `removePrototypeDeep` behavior.
///
/// # Arguments
///
/// * `object` - The object to sanitize recursively.
///
/// # Returns
///
/// A new object without the prototype-polluting properties at any depth.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_remove_prototype_deep, Value};
/// use umt_rust::obj;
///
/// let nested = obj!{
///     "safe" => obj!{"__proto__" => true, "value" => 1}
/// };
/// if let Value::Object(map) = nested {
///     let result = umt_remove_prototype_deep(&map);
///     if let Some(Value::Object(safe)) = result.get("safe") {
///         assert_eq!(safe.get("value"), Some(&Value::Int(1)));
///         assert_eq!(safe.get("__proto__"), None);
///     }
/// }
/// ```
pub fn umt_remove_prototype_deep(object: &HashMap<String, Value>) -> HashMap<String, Value> {
    sanitize_object(object)
}

fn sanitize_object(source: &HashMap<String, Value>) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    for (key, value) in source {
        if DANGEROUS_KEYS.contains(&key.as_str()) {
            continue;
        }
        result.insert(key.clone(), sanitize_value(value));
    }

    result
}

fn sanitize_value(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(sanitize_object(map)),
        Value::Array(items) => Value::Array(items.iter().map(sanitize_value).collect()),
        other => other.clone(),
    }
}
