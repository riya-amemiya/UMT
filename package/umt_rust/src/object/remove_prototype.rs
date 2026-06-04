use super::Value;
use std::collections::HashMap;

/// The keys excluded to prevent prototype pollution.
pub(crate) const DANGEROUS_KEYS: [&str; 3] = ["__proto__", "constructor", "prototype"];

/// Creates a new object with prototype-polluting properties removed.
///
/// The keys `__proto__`, `constructor` and `prototype` are excluded from the
/// shallow copy. Nested values are copied by clone (a shallow operation);
/// dangerous keys nested deeper are not touched, mirroring the TypeScript
/// `removePrototype` behavior.
///
/// # Arguments
///
/// * `object` - The object to sanitize.
///
/// # Returns
///
/// A new object without the prototype-polluting properties.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_remove_prototype, Value};
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("__proto__".to_string(), Value::Bool(true));
/// map.insert("a".to_string(), Value::Int(1));
///
/// let result = umt_remove_prototype(&map);
/// assert_eq!(result.get("a"), Some(&Value::Int(1)));
/// assert_eq!(result.get("__proto__"), None);
/// ```
pub fn umt_remove_prototype(object: &HashMap<String, Value>) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    for (key, value) in object {
        if !DANGEROUS_KEYS.contains(&key.as_str()) {
            result.insert(key.clone(), value.clone());
        }
    }

    result
}
