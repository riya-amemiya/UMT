use super::{Value, umt_remove_prototype_deep};
use std::collections::HashMap;

/// Creates a new vector of objects with prototype-polluting properties removed
/// recursively from each item.
///
/// Mirrors the TypeScript `removePrototypeMapDeep` behavior, applying
/// [`umt_remove_prototype_deep`] to every object while preserving order.
///
/// # Arguments
///
/// * `objects` - The objects to sanitize recursively.
///
/// # Returns
///
/// A new vector with deeply sanitized objects.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_remove_prototype_map_deep, Value};
/// use umt_rust::obj;
///
/// let dirty = obj!{"outer" => obj!{"__proto__" => true, "ok" => 1}};
/// if let Value::Object(map) = dirty {
///     let result = umt_remove_prototype_map_deep(&[map]);
///     if let Some(Value::Object(outer)) = result[0].get("outer") {
///         assert_eq!(outer.get("ok"), Some(&Value::Int(1)));
///         assert_eq!(outer.get("__proto__"), None);
///     }
/// }
/// ```
pub fn umt_remove_prototype_map_deep(
    objects: &[HashMap<String, Value>],
) -> Vec<HashMap<String, Value>> {
    objects.iter().map(umt_remove_prototype_deep).collect()
}
