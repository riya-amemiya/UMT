use super::Value;

/// Checks if a value is a plain object.
///
/// In Rust using `umt_rust::object::Value`, this checks if the value
/// is of the `Value::Object` variant.
///
/// # Arguments
///
/// * `value` - The value to check
///
/// # Returns
///
/// Returns true if the value is a plain object, false otherwise.
///
/// # Examples
///
/// ```
/// use umt_rust::object::{umt_is_plain_object, Value};
/// use std::collections::HashMap;
/// use umt_rust::obj;
///
/// let obj = obj!{"a" => 1};
/// assert!(umt_is_plain_object(&obj));
///
/// let arr = Value::Array(vec![]);
/// assert!(!umt_is_plain_object(&arr));
/// ```
pub fn umt_is_plain_object(value: &Value) -> bool {
    value.is_object()
}
