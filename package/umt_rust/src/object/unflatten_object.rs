use super::{Value, umt_set};
use std::collections::HashMap;

/// Reconstructs a nested object from a flat path-keyed object.
///
/// Mirrors the TypeScript `unflattenObject` behavior, splitting each key on
/// the separator and assigning the value at the resulting nested path.
///
/// # Prototype pollution warning
///
/// Internally uses [`umt_set`], which does not filter out prototype-polluting
/// keys. Sanitize user-controlled keys before calling.
///
/// # Arguments
///
/// * `flat` - The flat input keyed by joined paths.
/// * `separator` - The separator used in the path keys.
///
/// # Returns
///
/// A nested object.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_unflatten_object, Value};
/// use std::collections::HashMap;
///
/// let mut flat = HashMap::new();
/// flat.insert("a.b.c".to_string(), Value::Int(1));
///
/// let result = umt_unflatten_object(&flat, ".");
/// if let Some(Value::Object(a)) = result.get("a") {
///     if let Some(Value::Object(b)) = a.get("b") {
///         assert_eq!(b.get("c"), Some(&Value::Int(1)));
///     }
/// }
/// ```
pub fn umt_unflatten_object(
    flat: &HashMap<String, Value>,
    separator: &str,
) -> HashMap<String, Value> {
    let mut result: HashMap<String, Value> = HashMap::new();

    for (key, value) in flat {
        let segments: Vec<String> = key.split(separator).map(|s| s.to_string()).collect();
        umt_set(&mut result, &segments, value.clone());
    }

    result
}
