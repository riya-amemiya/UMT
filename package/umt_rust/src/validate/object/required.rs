//! Object validation - required
//! Returns a new object shape where every property validator is unwrapped from
//! its optional layer, so each one again requires a present value. Mirrors the
//! TypeScript `required` helper, which derives a `Required<T>` object validator
//! from an existing one by stripping the `optional()` wrapper from each property
//! of the source validator's `shape` map. It is the inverse of `partial`.

use std::collections::HashMap;

/// Unwraps every property validator of an optional shape so each one again
/// requires a present value, returning a new shape covering the same keys.
///
/// The source shape mirrors the shape produced by
/// [`umt_partial`](super::umt_partial): a map from property name to a
/// per-property validator that accepts an absent (`None`) value. Each validator
/// is unwrapped so the new validator delegates a present value to the original
/// by passing it as `Some`, mirroring the TypeScript source which replaces an
/// `optional()` wrapper with its inner validator. This undoes a `partial()`
/// wrapper exactly, and validators that were never optional are kept as-is by
/// the same delegation.
///
/// # Arguments
/// * `shape` - The source shape map; ownership is taken so the wrapped
///   validators can be moved into the unwrapping closures (validators are boxed
///   closures and are not cloneable).
///
/// # Returns
/// A new shape map covering the same keys, where every validator requires a
/// present value
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use umt_rust::validate::object::{umt_partial, umt_required};
///
/// let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
/// shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
///
/// let optional = umt_partial(shape);
/// let required = umt_required(optional);
/// let age = required.get("age").unwrap();
/// assert!(age(&30));
/// assert!(!age(&-1));
/// ```
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_required<V>(
    shape: HashMap<String, Box<dyn Fn(&Option<V>) -> bool>>,
) -> HashMap<String, Box<dyn Fn(&V) -> bool>>
where
    V: Clone + 'static,
{
    shape
        .into_iter()
        .map(|(key, validator)| {
            let unwrapped: Box<dyn Fn(&V) -> bool> =
                Box::new(move |value: &V| validator(&Some(value.clone())));
            (key, unwrapped)
        })
        .collect()
}
