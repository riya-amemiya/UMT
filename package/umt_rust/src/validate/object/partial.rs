//! Object validation - partial
//! Returns a new object shape where every property validator is wrapped to
//! accept an absent value. Mirrors the TypeScript `partial` helper, which
//! derives a `Partial<T>` object validator from an existing one by wrapping each
//! property of the source validator's `shape` map with `optional()`.

use std::collections::HashMap;

/// Wraps every property validator of a shape so each one accepts an absent
/// (`None`) value, returning a new shape covering the same keys.
///
/// The source shape mirrors the `validator.shape` map consumed by
/// [`umt_validate_object`](super::umt_validate_object): a map from property name
/// to a per-property validator. Each validator is wrapped so that `None` passes
/// (mirroring the TypeScript `optional()` wrapper accepting `undefined`) while a
/// present (`Some`) value is delegated to the original validator. The wrapping is
/// idempotent in spirit with the TypeScript source: re-wrapping a validator that
/// already passes on `None` keeps the pass-on-absence behavior.
///
/// # Arguments
/// * `shape` - The source shape map; ownership is taken so the original
///   validators can be moved into the wrapping closures (validators are boxed
///   closures and are not cloneable).
///
/// # Returns
/// A new shape map covering the same keys, where every validator accepts an
/// absent (`None`) value as valid
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use umt_rust::validate::object::umt_partial;
///
/// let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
/// shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
///
/// let partial = umt_partial(shape);
/// let age = partial.get("age").unwrap();
/// assert!(age(&None));
/// assert!(age(&Some(30)));
/// assert!(!age(&Some(-1)));
/// ```
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_partial<V>(
    shape: HashMap<String, Box<dyn Fn(&V) -> bool>>,
) -> HashMap<String, Box<dyn Fn(&Option<V>) -> bool>>
where
    V: 'static,
{
    shape
        .into_iter()
        .map(|(key, validator)| {
            let wrapped: Box<dyn Fn(&Option<V>) -> bool> =
                Box::new(move |value: &Option<V>| match value {
                    None => true,
                    Some(inner) => validator(inner),
                });
            (key, wrapped)
        })
        .collect()
}
