<<<<<<< HEAD
use super::Value;
use std::collections::HashMap;

/// Creates an object composed of the picked object properties.
///
/// # Arguments
///
/// * `object` - The source object
/// * `paths` - The property paths to pick
///
/// # Returns
///
/// Returns the new object.
pub fn umt_pick(object: &Value, paths: &[&str]) -> Value {
    match object {
        Value::Object(map) => {
            let mut result_map = HashMap::new();
            for path in paths {
                if let Some(val) = map.get(*path) {
                    result_map.insert(path.to_string(), val.clone());
                }
            }
            Value::Object(result_map)
        }
        _ => Value::Object(HashMap::new()),
    }
}
||||||| 55b8153
=======
use super::Value;
use std::collections::HashMap;

/// Creates a new object with only the specified properties from the source object.
///
/// # Arguments
///
/// * `object` - The source object to extract properties from
/// * `keys` - The property keys to extract
///
/// # Returns
///
/// A new HashMap containing only the specified properties.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_pick, Value};
/// use std::collections::HashMap;
///
/// let mut user = HashMap::new();
/// user.insert("id".to_string(), Value::Int(1));
/// user.insert("name".to_string(), Value::String("Alice".to_string()));
/// user.insert("age".to_string(), Value::Int(30));
///
/// let result = umt_pick(&user, &["id", "name"]);
///
/// assert_eq!(result.get("id"), Some(&Value::Int(1)));
/// assert_eq!(result.get("name"), Some(&Value::String("Alice".to_string())));
/// assert_eq!(result.get("age"), None); // not picked
/// ```
pub fn umt_pick(object: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    for key in keys {
        if let Some(value) = object.get(*key) {
            result.insert(key.to_string(), value.clone());
        }
    }

    result
}

/// Creates a new object with only the specified properties (using String keys).
///
/// # Arguments
///
/// * `object` - The source object to extract properties from
/// * `keys` - The property keys to extract as Strings
///
/// # Returns
///
/// A new HashMap containing only the specified properties.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_pick_string_keys, Value};
/// use std::collections::HashMap;
///
/// let mut obj = HashMap::new();
/// obj.insert("a".to_string(), Value::Int(1));
/// obj.insert("b".to_string(), Value::Int(2));
/// obj.insert("c".to_string(), Value::Int(3));
///
/// let keys = vec!["a".to_string(), "c".to_string()];
/// let result = umt_pick_string_keys(&obj, &keys);
///
/// assert_eq!(result.len(), 2);
/// assert_eq!(result.get("a"), Some(&Value::Int(1)));
/// assert_eq!(result.get("c"), Some(&Value::Int(3)));
/// ```
pub fn umt_pick_string_keys(
    object: &HashMap<String, Value>,
    keys: &[String],
) -> HashMap<String, Value> {
    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
    umt_pick(object, &key_refs)
}
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788
