use super::Value;
use std::collections::HashMap;

/// Creates a new object by deeply selecting properties from the source object based on specified keys.
///
/// # Arguments
///
/// * `object` - The source object to extract properties from
/// * `keys` - Property keys to extract. Can use dot notation for nested properties.
///
/// # Returns
///
/// A new HashMap containing only the specified properties with their nested structure preserved.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_pick_deep, Value};
/// use std::collections::HashMap;
///
/// let mut inner_b = HashMap::new();
/// inner_b.insert("c".to_string(), Value::Int(1));
/// inner_b.insert("d".to_string(), Value::Int(2));
///
/// let mut inner_a = HashMap::new();
/// inner_a.insert("b".to_string(), Value::Object(inner_b));
/// inner_a.insert("e".to_string(), Value::Int(3));
///
/// let mut obj = HashMap::new();
/// obj.insert("a".to_string(), Value::Object(inner_a));
/// obj.insert("f".to_string(), Value::Int(4));
///
/// let result = umt_pick_deep(&obj, &["a.b.c", "f"]);
///
/// // Check 'f' is picked
/// assert_eq!(result.get("f"), Some(&Value::Int(4)));
///
/// // Check 'a.b.c' is picked with nested structure
/// if let Some(Value::Object(a)) = result.get("a") {
///     if let Some(Value::Object(b)) = a.get("b") {
///         assert_eq!(b.get("c"), Some(&Value::Int(1)));
///         assert_eq!(b.get("d"), None); // not picked
///     } else {
///         panic!("Expected object for key 'b'");
///     }
/// } else {
///     panic!("Expected object for key 'a'");
/// }
/// ```
pub fn umt_pick_deep(object: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
    let mut result: HashMap<String, Value> = HashMap::new();

    for key in keys {
        pick_path(&mut result, object, key);
    }

    result
}

/// Copies the value at a dot-separated path from `source` into `result`,
/// walking both objects in a single pass like the TypeScript `pickDeep`.
///
/// A segment missing from the current source object is skipped, and an
/// intermediate segment that exists creates an empty object in the result
/// even when a later segment cannot be resolved.
fn pick_path(result: &mut HashMap<String, Value>, source: &HashMap<String, Value>, key: &str) {
    let last_index = key.split('.').count() - 1;
    let mut current = Some(source);
    let mut target = Some(result);

    for (index, part) in key.split('.').enumerate() {
        let Some(value) = current.and_then(|map| map.get(part)) else {
            continue;
        };

        if index == last_index {
            if let Some(map) = target.as_mut() {
                map.insert(part.to_string(), value.clone());
            }
            continue;
        }

        target = target.and_then(|map| {
            let nested = map
                .entry(part.to_string())
                .or_insert_with(|| Value::Object(HashMap::new()));
            if nested.is_null() {
                *nested = Value::Object(HashMap::new());
            }
            nested.as_object_mut()
        });
        current = value.as_object();
    }
}

/// Creates a new object by deeply selecting properties using String keys.
///
/// # Arguments
///
/// * `object` - The source object to extract properties from
/// * `keys` - Property keys to extract as Strings
///
/// # Returns
///
/// A new HashMap containing only the specified properties.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_pick_deep_string_keys, Value};
/// use std::collections::HashMap;
///
/// let mut inner = HashMap::new();
/// inner.insert("b".to_string(), Value::Int(1));
///
/// let mut obj = HashMap::new();
/// obj.insert("a".to_string(), Value::Object(inner));
///
/// let keys = vec!["a.b".to_string()];
/// let result = umt_pick_deep_string_keys(&obj, &keys);
///
/// if let Some(Value::Object(a)) = result.get("a") {
///     assert_eq!(a.get("b"), Some(&Value::Int(1)));
/// }
/// ```
pub fn umt_pick_deep_string_keys(
    object: &HashMap<String, Value>,
    keys: &[String],
) -> HashMap<String, Value> {
    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
    umt_pick_deep(object, &key_refs)
}
