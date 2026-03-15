use std::collections::HashMap;

use super::Value;

/// Creates an object with the same values as the given object, but
/// with keys transformed by the provided function.
///
/// # Arguments
///
/// * `object` - The source object.
/// * `function` - The function invoked per key. Receives (value, key).
///
/// # Returns
///
/// A new object with transformed keys.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use umt_rust::object::Value;
/// use umt_rust::object::map_keys::umt_map_keys;
///
/// let mut map = HashMap::new();
/// map.insert("a".to_string(), Value::Int(1));
/// map.insert("b".to_string(), Value::Int(2));
///
/// let result = umt_map_keys(&map, |_, key| key.to_uppercase());
///
/// assert_eq!(result.get("A"), Some(&Value::Int(1)));
/// assert_eq!(result.get("B"), Some(&Value::Int(2)));
/// ```
pub fn umt_map_keys<F>(object: &HashMap<String, Value>, function: F) -> HashMap<String, Value>
where
    F: Fn(&Value, &str) -> String,
{
    let mut result = HashMap::new();

    for (key, value) in object.iter() {
        result.insert(function(value, key), value.clone());
    }

    result
}
