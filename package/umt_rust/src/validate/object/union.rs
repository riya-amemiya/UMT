//! union validation utility
//!
//! Mirrors the TypeScript `union` validator, which composes several validators
//! into a single one that passes when *any* of them passes (logical OR). The
//! produced validator runs each member validator in order, returning success
//! with an empty message as soon as one accepts the value, and otherwise
//! propagating the *last* member's failure message.

use super::super::core::ValidateCoreReturnType;

/// Creates a union validator that passes if any of the given validators passes.
///
/// This mirrors the TypeScript `union`. Each member validator returns a
/// [`ValidateCoreReturnType`] (the same `{ validate, message, type }` shape used
/// by every validator in this crate); only its `validate` and `message` fields
/// are consulted. The returned closure runs the members in order: it returns a
/// successful result with an empty message as soon as a member accepts the
/// value, and if none accept it returns a failing result carrying the last
/// member's message. In both cases the original value is returned in
/// `type_value`.
///
/// Unlike the TypeScript original, which accepts heterogeneous validators over a
/// single dynamic value (e.g. `union(string(), number())`), Rust's type system
/// requires every member to validate the same input type `T`, so the members
/// share the closure's parameter type here.
///
/// # Arguments
/// * `validators` - Member validators, each returning a [`ValidateCoreReturnType`]
///
/// # Returns
/// A closure that validates a `T` and returns a `ValidateCoreReturnType<T>`
///
/// # Examples
/// ```
/// use umt_rust::validate::object::umt_union;
/// use umt_rust::validate::core::ValidateCoreReturnType;
///
/// let validator = umt_union(vec![
///     Box::new(|value: &i32| ValidateCoreReturnType {
///         validate: *value < 0,
///         message: "not negative".to_string(),
///         type_value: *value,
///     }) as Box<dyn Fn(&i32) -> ValidateCoreReturnType<i32>>,
///     Box::new(|value: &i32| ValidateCoreReturnType {
///         validate: *value > 10,
///         message: "not greater than ten".to_string(),
///         type_value: *value,
///     }),
/// ]);
///
/// let ok = validator(&-1);
/// assert!(ok.validate);
/// assert_eq!(ok.message, "");
///
/// let bad = validator(&5);
/// assert!(!bad.validate);
/// assert_eq!(bad.message, "not greater than ten");
/// ```
#[allow(clippy::type_complexity)]
pub fn umt_union<T, E>(
    validators: Vec<Box<dyn Fn(&T) -> ValidateCoreReturnType<E>>>,
) -> Box<dyn Fn(&T) -> ValidateCoreReturnType<T>>
where
    T: Clone + 'static,
    E: 'static,
{
    Box::new(move |value: &T| {
        let mut last_message = String::new();
        for validator in &validators {
            let result = validator(value);
            if result.validate {
                return ValidateCoreReturnType {
                    validate: true,
                    message: String::new(),
                    type_value: value.clone(),
                };
            }
            last_message = result.message;
        }

        ValidateCoreReturnType {
            validate: false,
            message: last_message,
            type_value: value.clone(),
        }
    })
}
