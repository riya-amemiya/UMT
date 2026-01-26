//! Array validation module
//! Provides validation functionality for arrays with type-specific validation rules

use super::core::ValidateCoreReturnType;

/// Validates an array of values
///
/// # Arguments
/// * `values` - The array to validate
/// * `validator` - Optional validation function to apply to each element
/// * `message` - Custom error message for array validation
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::array::umt_validate_array;
///
/// let arr = vec![1, 2, 3];
/// let result = umt_validate_array(&arr, None::<fn(&i32) -> bool>, None);
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_array<T: Clone, F>(
    values: &Vec<T>,
    validator: Option<F>,
    message: Option<&str>,
) -> ValidateCoreReturnType<Vec<T>>
where
    F: Fn(&T) -> bool,
{
    if let Some(validate_fn) = validator {
        for value in values {
            if !validate_fn(value) {
                return ValidateCoreReturnType {
                    validate: false,
                    message: message.unwrap_or("").to_string(),
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

/// Creates an array validator function
///
/// # Arguments
/// * `validator` - Validation function to apply to each element
/// * `message` - Custom error message
///
/// # Returns
/// A function that validates arrays
#[allow(clippy::type_complexity)]
pub fn umt_array_validator<T: Clone + 'static, F: Fn(&T) -> bool + 'static>(
    validator: F,
    message: Option<String>,
) -> Box<dyn Fn(&Vec<T>) -> ValidateCoreReturnType<Vec<T>>> {
    Box::new(move |values: &Vec<T>| {
        for value in values {
            if !validator(value) {
                return ValidateCoreReturnType {
                    validate: false,
                    message: message.clone().unwrap_or_default(),
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
