//! Literal validation utility
//!
//! Provides a validator that accepts exactly one primitive value compared with
//! JavaScript strict equality (`===`) semantics, mirroring the TypeScript
//! `literal()` validator.

use super::core::ValidateCoreReturnType;
use crate::object::Value;

fn strict_equal(value: &Value, expected: &Value) -> bool {
    match (value, expected) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => *a as f64 == *b,
        (Value::Float(a), Value::Float(b)) => a == b,
        _ => false,
    }
}

/// Creates a validator that accepts exactly one primitive value.
///
/// Values are compared like JavaScript `===`: `Int` and `Float` compare by
/// numeric value (so `Int(1)` matches `Float(1.0)` and `Float(0.0)` matches
/// `Float(-0.0)`), `Float(NaN)` never matches, and other variants must be the
/// same variant with an equal payload. `Value::Null` stands for both
/// JavaScript `null` and `undefined`. `Array` and `Object` literals never
/// match because JavaScript compares them by reference.
///
/// # Arguments
/// * `expected` - The only value accepted by the validator
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A function from `&Value` to a `ValidateCoreReturnType<Value>` whose
/// `type_value` is a clone of the validated value
///
/// # Examples
/// ```
/// use umt_rust::object::Value;
/// use umt_rust::validate::umt_literal;
///
/// let validator = umt_literal(Value::String("admin".to_string()), None);
/// assert!(validator(&Value::String("admin".to_string())).validate);
/// assert!(!validator(&Value::String("user".to_string())).validate);
///
/// let validator = umt_literal(Value::Bool(true), Some("must be true".to_string()));
/// assert_eq!(validator(&Value::Int(1)).message, "must be true");
/// ```
#[inline]
pub fn umt_literal(
    expected: Value,
    message: Option<String>,
) -> impl Fn(&Value) -> ValidateCoreReturnType<Value> {
    move |value: &Value| {
        let is_valid = strict_equal(value, &expected);
        ValidateCoreReturnType {
            validate: is_valid,
            message: if is_valid {
                String::new()
            } else {
                message.clone().unwrap_or_default()
            },
            type_value: value.clone(),
        }
    }
}
