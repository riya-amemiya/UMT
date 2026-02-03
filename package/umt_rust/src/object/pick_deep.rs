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
        let parts: Vec<&str> = key.split('.').collect();
        set_nested_value(&mut result, object, &parts);
    }

    result
}

/// Helper function to set a nested value in the result object.
fn set_nested_value(
    result: &mut HashMap<String, Value>,
    source: &HashMap<String, Value>,
    parts: &[&str],
) {
    if parts.is_empty() {
        return;
    }

    // Navigate to the value in the source
    let value = get_nested_value(source, parts);
    if value.is_none() {
        return;
    }
    let value = value.unwrap();

    // Build the nested structure in the result
    if parts.len() == 1 {
        result.insert(parts[0].to_string(), value);
    } else {
        // Get or create the nested object structure
        let first = parts[0];
        let remaining = &parts[1..];

        // Ensure the path exists in result
        let nested = result
            .entry(first.to_string())
            .or_insert_with(|| Value::Object(HashMap::new()));

        if let Value::Object(map) = nested {
            set_nested_value_recursive(map, remaining, value);
        }
    }
}

/// Recursively set a value at the given path in a nested structure.
fn set_nested_value_recursive(
    result: &mut HashMap<String, Value>,
    parts: &[&str],
    value: Value,
) {
    if parts.is_empty() {
        return;
    }

    if parts.len() == 1 {
        result.insert(parts[0].to_string(), value);
    } else {
        let first = parts[0];
        let remaining = &parts[1..];

        let nested = result
            .entry(first.to_string())
            .or_insert_with(|| Value::Object(HashMap::new()));

        if let Value::Object(map) = nested {
            set_nested_value_recursive(map, remaining, value);
        }
    }
}

/// Helper function to get a nested value from an object using a path.
fn get_nested_value(object: &HashMap<String, Value>, parts: &[&str]) -> Option<Value> {
    if parts.is_empty() {
        return None;
    }

    let first = parts[0];
    let value = object.get(first)?;

    if parts.len() == 1 {
        return Some(value.clone());
    }

    // Navigate deeper
    if let Value::Object(inner) = value {
        get_nested_value(inner, &parts[1..])
    } else {
        None
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
