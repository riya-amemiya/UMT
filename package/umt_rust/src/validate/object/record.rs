//! record validation utility
//!
//! Mirrors the TypeScript `record` validator, which validates dictionary
//! objects with dynamic keys by applying one validator to every key and
//! another to every value. The object is represented as a [`Value::Object`].

use super::super::core::ValidateCoreReturnType;
use crate::object::Value;
use crate::validate::array::ValueValidator;

/// Creates a record validator for objects with dynamic keys.
///
/// The returned closure fails with `message` when the value is not a
/// [`Value::Object`]. Otherwise it validates every key with `key_validator`
/// and then its value with `value_validator`, stopping at the first failure
/// and propagating that message. Keys are visited in ascending order so the
/// reported message is deterministic (a `HashMap` has no insertion order). On
/// success it returns an empty message. In every case `type_value` is a clone
/// of the validated value.
///
/// # Arguments
/// * `key_validator` - Validator applied to every key (for example the closure returned by `umt_string_validator` or `umt_one_of`)
/// * `value_validator` - Validator applied to every value
/// * `message` - Custom error message when the value is not an object
///
/// # Returns
/// A closure that validates a `&Value` and returns a `ValidateCoreReturnType<Value>`
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use umt_rust::object::Value;
/// use umt_rust::validate::core::ValidateCoreReturnType;
/// use umt_rust::validate::object::umt_record;
/// use umt_rust::validate::string::umt_one_of;
///
/// let validator = umt_record(
///     umt_one_of(&["en", "ja"], Some("unknown locale".to_string())),
///     |value: &Value| ValidateCoreReturnType {
///         validate: matches!(value, Value::String(_)),
///         message: "not a string".to_string(),
///         type_value: value.clone(),
///     },
///     Some("must be a record".to_string()),
/// );
///
/// let mut entries = HashMap::new();
/// entries.insert("en".to_string(), Value::String("Hello".to_string()));
/// assert!(validator(&Value::Object(entries.clone())).validate);
///
/// entries.insert("fr".to_string(), Value::String("Bonjour".to_string()));
/// assert_eq!(validator(&Value::Object(entries)).message, "unknown locale");
/// assert_eq!(validator(&Value::Null).message, "must be a record");
/// ```
pub fn umt_record<K, V, KF, VF>(
    key_validator: KF,
    value_validator: VF,
    message: Option<String>,
) -> ValueValidator<Value>
where
    KF: Fn(&str) -> ValidateCoreReturnType<K> + 'static,
    VF: Fn(&Value) -> ValidateCoreReturnType<V> + 'static,
{
    Box::new(move |value: &Value| {
        let Value::Object(entries) = value else {
            return ValidateCoreReturnType {
                validate: false,
                message: message.clone().unwrap_or_default(),
                type_value: value.clone(),
            };
        };
        let mut keys: Vec<&String> = entries.keys().collect();
        keys.sort();
        for key in keys {
            let key_result = key_validator(key);
            if !key_result.validate {
                return ValidateCoreReturnType {
                    validate: false,
                    message: key_result.message,
                    type_value: value.clone(),
                };
            }
            let value_result = value_validator(&entries[key]);
            if !value_result.validate {
                return ValidateCoreReturnType {
                    validate: false,
                    message: value_result.message,
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
