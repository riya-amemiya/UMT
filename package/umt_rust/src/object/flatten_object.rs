use super::Value;
use std::collections::HashMap;

/// Recursively flattens a nested object into a single-level object keyed by
/// joined paths.
///
/// Arrays and empty objects are kept as leaf values. Mirrors the TypeScript
/// `flattenObject` behavior where only non-empty plain objects are recursed
/// into.
///
/// # Arguments
///
/// * `object` - The source nested object.
/// * `separator` - The separator used to join path segments.
///
/// # Returns
///
/// A flat object keyed by joined paths.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_flatten_object, Value};
/// use umt_rust::obj;
///
/// let nested = obj!{"a" => obj!{"b" => obj!{"c" => 1}}};
/// if let Value::Object(map) = nested {
///     let flat = umt_flatten_object(&map, ".");
///     assert_eq!(flat.get("a.b.c"), Some(&Value::Int(1)));
/// }
/// ```
pub fn umt_flatten_object(
    object: &HashMap<String, Value>,
    separator: &str,
) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    walk(object, "", separator, &mut result);
    result
}

fn walk(
    current: &HashMap<String, Value>,
    prefix: &str,
    separator: &str,
    result: &mut HashMap<String, Value>,
) {
    for (key, value) in current {
        let next_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{}{}{}", prefix, separator, key)
        };

        match value {
            Value::Object(inner) if !inner.is_empty() => {
                walk(inner, &next_key, separator, result);
            }
            _ => {
                result.insert(next_key, value.clone());
            }
        }
    }
}
