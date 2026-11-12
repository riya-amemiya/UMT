use super::Value;
use std::collections::HashMap;

/// Creates a new object by deeply removing properties from the source object
/// based on specified keys. This is the counterpart of [`super::umt_pick_deep`].
///
/// Each key is removed with [`super::umt_unset_path`], so the source object is
/// not mutated. Keys that do not exist are ignored.
///
/// # Arguments
///
/// * `object` - The source object to remove properties from
/// * `keys` - Property keys to remove. Can use dot notation for nested properties.
///
/// # Returns
///
/// A new HashMap without the specified properties.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_omit_deep, Value};
/// use umt_rust::obj;
///
/// let map = obj!{
///     "a" => obj!{"b" => obj!{"c" => 1, "d" => 2}, "e" => 3},
///     "f" => 4
/// }
/// .into_object()
/// .unwrap();
/// let result = umt_omit_deep(&map, &["a.b.c", "f"]);
///
/// assert_eq!(
///     Value::Object(result),
///     obj!{"a" => obj!{"b" => obj!{"d" => 2}, "e" => 3}}
/// );
/// ```
pub fn umt_omit_deep(object: &HashMap<String, Value>, keys: &[&str]) -> HashMap<String, Value> {
    keys.iter().fold(object.clone(), |result, key| {
        super::umt_unset_path(&result, key)
    })
}

/// Creates a new object by deeply removing properties using String keys.
///
/// # Arguments
///
/// * `object` - The source object to remove properties from
/// * `keys` - Property keys to remove as Strings
///
/// # Returns
///
/// A new HashMap without the specified properties.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_omit_deep_string_keys, Value};
/// use umt_rust::obj;
///
/// let map = obj!{"a" => obj!{"b" => 1, "c" => 2}}.into_object().unwrap();
/// let result = umt_omit_deep_string_keys(&map, &["a.b".to_string()]);
///
/// assert_eq!(Value::Object(result), obj!{"a" => obj!{"c" => 2}});
/// ```
pub fn umt_omit_deep_string_keys(
    object: &HashMap<String, Value>,
    keys: &[String],
) -> HashMap<String, Value> {
    let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
    umt_omit_deep(object, &key_refs)
}
