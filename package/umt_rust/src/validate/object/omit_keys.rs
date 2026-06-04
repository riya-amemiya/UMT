//! Object validation - omit_keys
//! Returns a new object shape that drops the specified keys from the source
//! shape. Mirrors the TypeScript `omitKeys` helper, which derives an
//! `Omit<T, K>` object validator from an existing one by copying every entry of
//! the source validator's `shape` map except the omitted keys.

use std::collections::HashMap;

/// Removes the given keys from an existing object shape and returns a new shape
/// covering the remaining keys.
///
/// The source shape mirrors the `validator.shape` map consumed by
/// [`umt_validate_object`](super::umt_validate_object): a map from property name
/// to a per-property validator. Every entry whose key is not in `keys` is moved
/// into the new shape, matching the TypeScript source which copies
/// `sourceShape[key]` for each key not present in the omitted set. Requested keys
/// absent from the source shape have no effect.
///
/// # Arguments
/// * `shape` - The source shape map; ownership is taken so the retained
///   validators can be moved into the new shape (validators are boxed closures
///   and are not cloneable).
/// * `keys` - The keys to drop from the new shape
///
/// # Returns
/// A new shape map containing every key of the source shape except the omitted
/// keys
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use umt_rust::validate::object::{umt_omit_keys, umt_validate_object};
///
/// let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
/// shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
/// shape.insert("score".to_string(), Box::new(|v: &i32| *v <= 100));
///
/// let omitted = umt_omit_keys(shape, &["score".to_string()]);
/// assert!(omitted.contains_key("age"));
/// assert!(!omitted.contains_key("score"));
///
/// let mut value: HashMap<String, i32> = HashMap::new();
/// value.insert("age".to_string(), 30);
/// let result = umt_validate_object(&value, Some(&omitted), None);
/// assert!(result.validate);
/// ```
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_omit_keys<V>(
    shape: HashMap<String, Box<dyn Fn(&V) -> bool>>,
    keys: &[String],
) -> HashMap<String, Box<dyn Fn(&V) -> bool>> {
    let omitted: std::collections::HashSet<&String> = keys.iter().collect();
    shape
        .into_iter()
        .filter(|(key, _)| !omitted.contains(key))
        .collect()
}
