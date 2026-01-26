//! Object validation module
//! Provides validation functionality for objects (HashMap) with property-specific validation rules

use std::collections::HashMap;

use super::core::ValidateCoreReturnType;

/// Validates a HashMap with optional property validators
///
/// # Arguments
/// * `value` - The HashMap to validate
/// * `validators` - Map of property validators
/// * `message` - Custom error message for object type validation
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::object::umt_validate_object;
/// use std::collections::HashMap;
///
/// let mut obj = HashMap::new();
/// obj.insert("name".to_string(), "test".to_string());
/// let result = umt_validate_object(&obj, None, None);
/// assert!(result.validate);
/// ```
#[inline]
#[allow(clippy::type_complexity)]
pub fn umt_validate_object<V: Clone>(
    value: &HashMap<String, V>,
    validators: Option<&HashMap<String, Box<dyn Fn(&V) -> bool>>>,
    message: Option<&str>,
) -> ValidateCoreReturnType<HashMap<String, V>> {
    if let Some(validator_map) = validators {
        for (key, validator) in validator_map {
            if let Some(val) = value.get(key) {
                if !validator(val) {
                    return ValidateCoreReturnType {
                        validate: false,
                        message: message.unwrap_or("").to_string(),
                        type_value: value.clone(),
                    };
                }
            } else {
                return ValidateCoreReturnType {
                    validate: false,
                    message: format!("Missing required field: {}", key),
                    type_value: value.clone(),
                };
            }
        }
    }

    ValidateCoreReturnType {
        validate: true,
        message: String::new(),
        type_value: value.clone(),
    }
}

/// Creates an optional validator wrapper
///
/// The optional validator returns true for None/undefined values
/// and applies the inner validator for Some values.
///
/// # Arguments
/// * `validator` - The inner validator function
///
/// # Returns
/// A function that validates optional values
#[allow(clippy::type_complexity)]
pub fn umt_optional<T, F>(validator: F) -> Box<dyn Fn(&Option<T>) -> bool>
where
    F: Fn(&T) -> bool + 'static,
    T: 'static,
{
    Box::new(move |value: &Option<T>| match value {
        None => true,
        Some(v) => validator(v),
    })
}

/// Result type for optional validation
#[derive(Debug, Clone)]
pub struct OptionalValidateCoreReturnType<T> {
    pub validate: bool,
    pub message: String,
    pub type_value: Option<T>,
}

/// Validates an optional value
///
/// # Arguments
/// * `value` - The optional value to validate
/// * `validator` - The validation function to apply if value is Some
///
/// # Returns
/// An `OptionalValidateCoreReturnType` containing the validation result
#[inline]
pub fn umt_validate_optional<T: Clone, F>(
    value: &Option<T>,
    validator: F,
) -> OptionalValidateCoreReturnType<T>
where
    F: Fn(&T) -> ValidateCoreReturnType<T>,
{
    match value {
        None => OptionalValidateCoreReturnType {
            validate: true,
            message: String::new(),
            type_value: None,
        },
        Some(v) => {
            let result = validator(v);
            OptionalValidateCoreReturnType {
                validate: result.validate,
                message: result.message,
                type_value: Some(result.type_value),
            }
        }
    }
}
