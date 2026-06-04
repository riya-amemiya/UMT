use super::Value;
use std::collections::HashMap;

/// Creates a new object containing only entries for which the predicate
/// returns false.
///
/// Mirrors the TypeScript `omitBy` behavior. The predicate receives the
/// value and key for each entry; entries where it returns `true` are dropped.
///
/// # Arguments
///
/// * `object` - The source object.
/// * `predicate` - The rejection predicate, receiving (value, key).
///
/// # Returns
///
/// A new object with the remaining entries.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_omit_by, Value};
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a".to_string(), Value::Int(1));
/// map.insert("b".to_string(), Value::Null);
/// map.insert("c".to_string(), Value::Int(3));
///
/// let result = umt_omit_by(&map, |value, _| value.is_null());
/// assert_eq!(result.get("a"), Some(&Value::Int(1)));
/// assert_eq!(result.get("c"), Some(&Value::Int(3)));
/// assert_eq!(result.get("b"), None);
/// ```
pub fn umt_omit_by<F>(object: &HashMap<String, Value>, predicate: F) -> HashMap<String, Value>
where
    F: Fn(&Value, &str) -> bool,
{
    let mut result = HashMap::new();

    for (key, value) in object {
        if !predicate(value, key) {
            result.insert(key.clone(), value.clone());
        }
    }

    result
}
