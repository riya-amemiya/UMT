//! arrayOf validation utility
//!
//! Mirrors the TypeScript `arrayOf` validator, which builds an array validator
//! from a single per-element validator. The produced validator runs the element
//! validator over every element, short-circuiting on the first failure and
//! propagating that element's message, and preserves the original array in the
//! returned `type_value` field.

use super::super::core::ValidateCoreReturnType;

/// Creates an array validator that validates every element with a single validator.
///
/// This mirrors the TypeScript `arrayOf`. The element validator returns a
/// [`ValidateCoreReturnType`] (the same `{ validate, message, type }` shape used
/// by every validator in this crate); only its `validate` and `message` fields
/// are consulted. The returned closure validates a whole `Vec`, stopping at the
/// first element whose `validate` is `false` and propagating that element's
/// message. On success it returns an empty message. In both cases the original
/// array is returned unchanged in `type_value`.
///
/// Unlike the TypeScript original there is no runtime "not an array" branch:
/// Rust guarantees the argument is a `Vec` at compile time, so the `isArray`
/// guard collapses away and the custom `message` argument has no use, hence it
/// is omitted here.
///
/// # Arguments
/// * `validator` - Validator applied to each element, returning a [`ValidateCoreReturnType`]
///
/// # Returns
/// A closure that validates a `Vec<T>` and returns a `ValidateCoreReturnType<Vec<T>>`
///
/// # Examples
/// ```
/// use umt_rust::validate::array::umt_array_of;
/// use umt_rust::validate::core::ValidateCoreReturnType;
///
/// let validator = umt_array_of(|value: &i32| ValidateCoreReturnType {
///     validate: *value >= 0,
///     message: if *value >= 0 { String::new() } else { "negative".to_string() },
///     type_value: *value,
/// });
///
/// let ok = validator(&vec![0, 1, 2]);
/// assert!(ok.validate);
/// assert_eq!(ok.message, "");
///
/// let bad = validator(&vec![1, -1, -2]);
/// assert!(!bad.validate);
/// assert_eq!(bad.message, "negative");
/// ```
#[allow(clippy::type_complexity)]
pub fn umt_array_of<T, E, F>(validator: F) -> Box<dyn Fn(&Vec<T>) -> ValidateCoreReturnType<Vec<T>>>
where
    T: Clone + 'static,
    F: Fn(&T) -> ValidateCoreReturnType<E> + 'static,
{
    Box::new(move |values: &Vec<T>| {
        for value in values {
            let result = validator(value);
            if !result.validate {
                return ValidateCoreReturnType {
                    validate: false,
                    message: result.message,
                    type_value: values.clone(),
                };
            }
        }

        ValidateCoreReturnType {
            validate: true,
            message: String::new(),
            type_value: values.clone(),
        }
    })
}
