use super::{Value, umt_remove_prototype};
use std::collections::HashMap;

/// Creates a new vector of objects with prototype-polluting properties removed
/// from each item (shallow).
///
/// Mirrors the TypeScript `removePrototypeMap` behavior, applying
/// [`umt_remove_prototype`] to every object while preserving order.
///
/// # Arguments
///
/// * `objects` - The objects to sanitize.
///
/// # Returns
///
/// A new vector with shallowly sanitized objects.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_remove_prototype_map, Value};
/// use std::collections::HashMap;
///
/// let mut first = HashMap::new();
/// first.insert("__proto__".to_string(), Value::Bool(true));
/// first.insert("a".to_string(), Value::Int(1));
///
/// let result = umt_remove_prototype_map(&[first]);
/// assert_eq!(result[0].get("a"), Some(&Value::Int(1)));
/// assert_eq!(result[0].get("__proto__"), None);
/// ```
pub fn umt_remove_prototype_map(objects: &[HashMap<String, Value>]) -> Vec<HashMap<String, Value>> {
    objects.iter().map(umt_remove_prototype).collect()
}
