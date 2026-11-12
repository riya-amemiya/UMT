//! tuple validation utility
//!
//! Mirrors the TypeScript `tuple` validator, which validates a fixed-length
//! heterogeneous array position by position. Rust vectors are homogeneous, so
//! the tuple is represented as a [`Value::Array`].

use super::super::core::ValidateCoreReturnType;
use crate::object::Value;

/// A boxed validator that checks a [`Value`] and returns a result typed `T`.
pub type ValueValidator<T> = Box<dyn Fn(&Value) -> ValidateCoreReturnType<T>>;

/// Creates a tuple validator for fixed-length arrays whose elements are
/// validated with the validator at the same index.
///
/// The returned closure fails with `message` when the value is not a
/// [`Value::Array`] or when its length differs from the number of validators.
/// Otherwise it runs every validator against the element at its index,
/// stopping at the first failure and propagating that element's message. On
/// success it returns an empty message. In every case `type_value` is a clone
/// of the validated value.
///
/// # Arguments
/// * `validators` - Validators applied to the element at the same index
/// * `message` - Custom error message when the value is not an array of the expected length
///
/// # Returns
/// A closure that validates a `&Value` and returns a `ValidateCoreReturnType<Value>`
///
/// # Examples
/// ```
/// use umt_rust::object::Value;
/// use umt_rust::validate::array::umt_tuple;
/// use umt_rust::validate::core::ValidateCoreReturnType;
///
/// let is_string = |value: &Value| ValidateCoreReturnType {
///     validate: matches!(value, Value::String(_)),
///     message: "not a string".to_string(),
///     type_value: value.clone(),
/// };
/// let is_int = |value: &Value| ValidateCoreReturnType {
///     validate: matches!(value, Value::Int(_)),
///     message: "not an int".to_string(),
///     type_value: value.clone(),
/// };
/// let validator = umt_tuple(
///     vec![
///         Box::new(is_string) as Box<dyn Fn(&Value) -> ValidateCoreReturnType<Value>>,
///         Box::new(is_int),
///     ],
///     Some("must be [string, int]".to_string()),
/// );
///
/// let ok = Value::Array(vec![Value::String("a".to_string()), Value::Int(1)]);
/// assert!(validator(&ok).validate);
///
/// let bad = Value::Array(vec![Value::String("a".to_string()), Value::Null]);
/// assert_eq!(validator(&bad).message, "not an int");
///
/// let short = Value::Array(vec![Value::String("a".to_string())]);
/// assert_eq!(validator(&short).message, "must be [string, int]");
/// ```
pub fn umt_tuple<E>(
    validators: Vec<ValueValidator<E>>,
    message: Option<String>,
) -> ValueValidator<Value>
where
    E: 'static,
{
    Box::new(move |value: &Value| {
        let elements = match value {
            Value::Array(elements) if elements.len() == validators.len() => elements,
            _ => {
                return ValidateCoreReturnType {
                    validate: false,
                    message: message.clone().unwrap_or_default(),
                    type_value: value.clone(),
                };
            }
        };
        for (validator, element) in validators.iter().zip(elements) {
            let result = validator(element);
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
