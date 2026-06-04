//! Set validation module
//! Provides validation for sets. The validator can optionally delegate to a
//! per-element validator, mirroring how the array module validates each element
//! of an array. Iteration short-circuits at the first failing element and
//! surfaces that element's message.

use std::collections::HashSet;
use std::hash::Hash;

use super::core::ValidateCoreReturnType;

/// Validates a set of values
///
/// When a per-element validator is supplied, every element of the set must
/// satisfy it; iteration short-circuits at the first failure and surfaces the
/// failing element's message.
///
/// # Arguments
/// * `values` - The set to validate
/// * `validator` - Optional validation function applied to every element. It
///   returns a [`ValidateCoreReturnType`] so the failing element's message can
///   be propagated.
///
/// # Returns
/// A [`ValidateCoreReturnType`] containing the validation result
///
/// # Examples
/// ```
/// use std::collections::HashSet;
/// use umt_rust::validate::set::umt_validate_set;
///
/// let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
/// let result = umt_validate_set(&set, None::<fn(&i32) -> _>);
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_set<T, F>(
    values: &HashSet<T>,
    validator: Option<F>,
) -> ValidateCoreReturnType<HashSet<T>>
where
    T: Clone + Eq + Hash,
    F: Fn(&T) -> ValidateCoreReturnType<T>,
{
    if let Some(validate_fn) = validator {
        for value in values {
            let result = validate_fn(value);
            if !result.validate {
                return ValidateCoreReturnType {
                    validate: false,
                    message: result.message,
                    type_value: values.clone(),
                };
            }
        }
    }

    ValidateCoreReturnType {
        validate: true,
        message: String::new(),
        type_value: values.clone(),
    }
}

/// Creates a set validator function
///
/// Mirrors the `setOf` factory from the TypeScript source: every element of the
/// set must satisfy the supplied validator, and the first failing element's
/// message is propagated.
///
/// # Arguments
/// * `validator` - Validation function applied to every element
///
/// # Returns
/// A function that validates sets
#[allow(clippy::type_complexity)]
pub fn umt_set_validator<T, F>(
    validator: F,
) -> Box<dyn Fn(&HashSet<T>) -> ValidateCoreReturnType<HashSet<T>>>
where
    T: Clone + Eq + Hash + 'static,
    F: Fn(&T) -> ValidateCoreReturnType<T> + 'static,
{
    Box::new(move |values: &HashSet<T>| {
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
