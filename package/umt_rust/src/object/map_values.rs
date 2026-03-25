use std::collections::HashMap;

use super::Value;

/// Creates an object with the same keys as the given object, but
/// with values transformed by the provided function.
///
/// # Arguments
///
/// * `object` - The source object.
/// * `function` - The function invoked per value. Receives (value, key).
///
/// # Returns
///
/// A new object with transformed values.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use umt_rust::object::Value;
/// use umt_rust::object::map_values::umt_map_values;
///
/// let mut map = HashMap::new();
/// map.insert("a".to_string(), Value::Int(1));
/// map.insert("b".to_string(), Value::Int(2));
///
/// let result = umt_map_values(&map, |value, _| match value {
///     Value::Int(v) => Value::Int(v * 2),
///     other => other.clone(),
/// });
///
/// assert_eq!(result.get("a"), Some(&Value::Int(2)));
/// assert_eq!(result.get("b"), Some(&Value::Int(4)));
/// ```
pub fn umt_map_values<F>(object: &HashMap<String, Value>, function: F) -> HashMap<String, Value>
where
    F: Fn(&Value, &str) -> Value,
{
    let mut result = HashMap::new();

    for (key, value) in object.iter() {
        result.insert(key.clone(), function(value, key));
    }

    result
}
