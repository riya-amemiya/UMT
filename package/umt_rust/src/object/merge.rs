use super::Value;
use std::collections::HashMap;

/// Merges multiple objects into a single object (shallow merge).
///
/// # Arguments
///
/// * `target` - The target object to merge into
/// * `sources` - The source objects to merge from
///
/// # Returns
///
/// A new merged HashMap.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_merge, Value};
/// use std::collections::HashMap;
///
/// let mut target = HashMap::new();
/// target.insert("a".to_string(), Value::Int(1));
/// target.insert("b".to_string(), Value::Int(2));
///
/// let mut source = HashMap::new();
/// source.insert("b".to_string(), Value::Int(3));
/// source.insert("c".to_string(), Value::Int(4));
///
/// let result = umt_merge(&target, &[&source]);
///
/// assert_eq!(result.get("a"), Some(&Value::Int(1)));
/// assert_eq!(result.get("b"), Some(&Value::Int(3))); // overwritten by source
/// assert_eq!(result.get("c"), Some(&Value::Int(4)));
/// ```
pub fn umt_merge(
    target: &HashMap<String, Value>,
    sources: &[&HashMap<String, Value>],
) -> HashMap<String, Value> {
    let mut result = target.clone();

    for source in sources {
        for (key, value) in *source {
            result.insert(key.clone(), value.clone());
        }
    }

    result
}

/// Merges two objects into a single object (shallow merge).
///
/// # Arguments
///
/// * `target` - The target object to merge into
/// * `source` - The source object to merge from
///
/// # Returns
///
/// A new merged HashMap.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_merge_two, Value};
/// use std::collections::HashMap;
///
/// let mut target = HashMap::new();
/// target.insert("a".to_string(), Value::Int(1));
///
/// let mut source = HashMap::new();
/// source.insert("b".to_string(), Value::Int(2));
///
/// let result = umt_merge_two(&target, &source);
///
/// assert_eq!(result.get("a"), Some(&Value::Int(1)));
/// assert_eq!(result.get("b"), Some(&Value::Int(2)));
/// ```
pub fn umt_merge_two(
    target: &HashMap<String, Value>,
    source: &HashMap<String, Value>,
) -> HashMap<String, Value> {
    umt_merge(target, &[source])
}
