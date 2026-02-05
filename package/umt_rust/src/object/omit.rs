use super::Value;
use std::collections::HashMap;

/// Creates an object without the specified keys.
///
/// # Arguments
///
/// * `object` - The source object
/// * `keys` - The keys to omit
///
/// # Returns
///
/// A new HashMap without the specified keys.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_omit, Value};
/// use std::collections::HashMap;
///
/// let mut user = HashMap::new();
/// user.insert("id".to_string(), Value::Int(1));
/// user.insert("name".to_string(), Value::String("Alice".to_string()));
/// user.insert("password".to_string(), Value::String("secret".to_string()));
/// user.insert("email".to_string(), Value::String("alice@example.com".to_string()));
///
/// let result = umt_omit(&user, &["password"]);
///
/// assert_eq!(result.get("id"), Some(&Value::Int(1)));
/// assert_eq!(result.get("name"), Some(&Value::String("Alice".to_string())));
/// assert_eq!(result.get("email"), Some(&Value::String("alice@example.com".to_string())));
/// assert_eq!(result.get("password"), None); // omitted
/// ```
pub fn umt_omit(object: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
    let keys_set: std::collections::HashSet<&str> = keys.iter().copied().collect();
    let mut result = HashMap::new();

    for (key, value) in object {
        if !keys_set.contains(key.as_str()) {
            result.insert(key.clone(), value.clone());
        }
    }

    result
}

/// Creates an object without the specified keys (using String keys).
///
/// # Arguments
///
/// * `object` - The source object
/// * `keys` - The keys to omit as Strings
///
/// # Returns
///
/// A new HashMap without the specified keys.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_omit_string_keys, Value};
/// use std::collections::HashMap;
///
/// let mut obj = HashMap::new();
/// obj.insert("a".to_string(), Value::Int(1));
/// obj.insert("b".to_string(), Value::Int(2));
/// obj.insert("c".to_string(), Value::Int(3));
///
/// let keys = vec!["b".to_string()];
/// let result = umt_omit_string_keys(&obj, &keys);
///
/// assert_eq!(result.len(), 2);
/// assert_eq!(result.get("a"), Some(&Value::Int(1)));
/// assert_eq!(result.get("c"), Some(&Value::Int(3)));
/// assert_eq!(result.get("b"), None);
/// ```
pub fn umt_omit_string_keys(
    object: &HashMap<String, Value>,
    keys: &[String],
) -> HashMap<String, Value> {
    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
    umt_omit(object, &key_refs)
}
