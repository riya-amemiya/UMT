//! intersection validation utility
//!
//! Mirrors the TypeScript `intersection` validator, which composes several
//! validators into a single one that passes only when *all* of them pass
//! (logical AND). The produced validator runs each member validator in order,
//! returning the *first* member's failure as soon as one rejects the value, and
//! otherwise succeeding with an empty message.

use super::super::core::ValidateCoreReturnType;

/// Creates an intersection validator that passes only if all of the given
/// validators pass.
///
/// This mirrors the TypeScript `intersection`. Each member validator returns a
/// [`ValidateCoreReturnType`] (the same `{ validate, message, type }` shape used
/// by every validator in this crate); only its `validate` and `message` fields
/// are consulted. The returned closure runs the members in order: it returns the
/// first failing result (propagating that member's message) as soon as a member
/// rejects the value, and if every member accepts it returns a successful result
/// with an empty message. In both cases the original value is returned in
/// `type_value`.
///
/// Unlike the TypeScript original, which accepts heterogeneous validators over a
/// single dynamic value, Rust's type system requires every member to validate
/// the same input type `T`, so the members share the closure's parameter type
/// here.
///
/// # Arguments
/// * `validators` - Member validators, each returning a [`ValidateCoreReturnType`]
///
/// # Returns
/// A closure that validates a `T` and returns a `ValidateCoreReturnType<T>`
///
/// # Examples
/// ```
/// use umt_rust::validate::object::umt_intersection;
/// use umt_rust::validate::core::ValidateCoreReturnType;
///
/// let validator = umt_intersection(vec![
///     Box::new(|value: &i32| ValidateCoreReturnType {
///         validate: *value > 0,
///         message: "not positive".to_string(),
///         type_value: *value,
///     }) as Box<dyn Fn(&i32) -> ValidateCoreReturnType<i32>>,
///     Box::new(|value: &i32| ValidateCoreReturnType {
///         validate: *value < 100,
///         message: "not less than one hundred".to_string(),
///         type_value: *value,
///     }),
/// ]);
///
/// let ok = validator(&42);
/// assert!(ok.validate);
/// assert_eq!(ok.message, "");
///
/// let bad = validator(&-1);
/// assert!(!bad.validate);
/// assert_eq!(bad.message, "not positive");
/// ```
#[allow(clippy::type_complexity)]
pub fn umt_intersection<T, E>(
    validators: Vec<Box<dyn Fn(&T) -> ValidateCoreReturnType<E>>>,
) -> Box<dyn Fn(&T) -> ValidateCoreReturnType<T>>
where
    T: Clone + 'static,
    E: 'static,
{
    Box::new(move |value: &T| {
        for validator in &validators {
            let result = validator(value);
            if !result.validate {
                return ValidateCoreReturnType {
                    validate: false,
                    message: result.message,
                    type_value: value.clone(),
                };
            }
        }

        ValidateCoreReturnType {
            validate: true,
            message: String::new(),
            type_value: value.clone(),
        }
    })
}
