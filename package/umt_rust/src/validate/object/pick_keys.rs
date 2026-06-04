//! Object validation - pick_keys
//! Returns a new object shape that only retains the specified keys of the
//! source shape. Mirrors the TypeScript `pickKeys` helper, which derives a
//! `Pick<T, K>` object validator from an existing one by copying the picked
//! entries out of the source validator's `shape` map.

use std::collections::HashMap;

/// Picks the given keys from an existing object shape and returns a new shape
/// covering only those keys.
///
/// The source shape mirrors the `validator.shape` map consumed by
/// [`umt_validate_object`](super::umt_validate_object): a map from property name
/// to a per-property validator. Keys are visited in the order supplied; a
/// requested key that is absent from the source shape is skipped, matching the
/// TypeScript source which copies `sourceShape[key]` for each requested key.
///
/// # Arguments
/// * `shape` - The source shape map; ownership is taken so the picked validators
///   can be moved into the new shape (validators are boxed closures and are not
///   cloneable).
/// * `keys` - The keys to retain on the new shape
///
/// # Returns
/// A new shape map containing only the picked keys that existed in the source
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use umt_rust::validate::object::{umt_pick_keys, umt_validate_object};
///
/// let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
/// shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
/// shape.insert("score".to_string(), Box::new(|v: &i32| *v <= 100));
///
/// let picked = umt_pick_keys(shape, &["age".to_string()]);
/// assert!(picked.contains_key("age"));
/// assert!(!picked.contains_key("score"));
///
/// let mut value: HashMap<String, i32> = HashMap::new();
/// value.insert("age".to_string(), 30);
/// let result = umt_validate_object(&value, Some(&picked), None);
/// assert!(result.validate);
/// ```
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_pick_keys<V>(
    mut shape: HashMap<String, Box<dyn Fn(&V) -> bool>>,
    keys: &[String],
) -> HashMap<String, Box<dyn Fn(&V) -> bool>> {
    let mut next_shape: HashMap<String, Box<dyn Fn(&V) -> bool>> = HashMap::new();
    for key in keys {
        if let Some(validator) = shape.remove(key) {
            next_shape.insert(key.clone(), validator);
        }
    }
    next_shape
}
