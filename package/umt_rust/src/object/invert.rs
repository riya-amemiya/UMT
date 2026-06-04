use super::Value;
use std::collections::HashMap;

/// Creates a new object with keys and values swapped.
///
/// Each value is coerced to a string to form the new key, and the original
/// key becomes the new value (as a `Value::String`). If multiple keys share
/// the same value, the last entry encountered wins, mirroring the TypeScript
/// `invert` behavior.
///
/// # Arguments
///
/// * `object` - The source object.
///
/// # Returns
///
/// A new object with keys and values swapped.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_invert, Value};
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a".to_string(), Value::Int(1));
/// map.insert("b".to_string(), Value::Int(2));
///
/// let result = umt_invert(&map);
/// assert_eq!(result.get("1"), Some(&Value::String("a".to_string())));
/// assert_eq!(result.get("2"), Some(&Value::String("b".to_string())));
/// ```
pub fn umt_invert(object: &HashMap<String, Value>) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    for (key, value) in object {
        result.insert(value_to_key(value), Value::String(key.clone()));
    }

    result
}

/// Converts a value into the string key it would produce in JavaScript when
/// used as an object key.
fn value_to_key(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        other => other.to_string(),
    }
}
