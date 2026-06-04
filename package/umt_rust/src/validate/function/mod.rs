//! Function validation module
//! Provides a validator for callable values along with an `implement`
//! helper that wraps a concrete function with runtime validation of its
//! input and output.
//!
//! The runtime validator only confirms that the value is callable, since
//! argument and return-value contracts can only be checked when the
//! function is actually invoked. Use [`umt_func_implement`] to obtain a
//! wrapper that asserts the input/output validators on every call.

use super::core::{ValidateCoreReturnType, ValidateReturnType};

/// Validates a callable value.
///
/// In Rust a value typed as `Fn` is callable by construction, so this
/// always reports `validate: true`, mirroring the TypeScript `func()`
/// validator that only enforces the value is a function. Argument and
/// return-value contracts are checked by [`umt_func_implement`].
///
/// # Arguments
/// * `value` - The callable value to validate
/// * `message` - Custom error message carried on the result
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::function::umt_validate_func;
///
/// let result = umt_validate_func(|x: i32| x + 1, None);
/// assert!(result.validate);
/// assert_eq!((result.type_value)(1), 2);
/// ```
#[inline]
pub fn umt_validate_func<F>(value: F, message: Option<&str>) -> ValidateCoreReturnType<F> {
    ValidateCoreReturnType {
        validate: true,
        message: message.unwrap_or("").to_string(),
        type_value: value,
    }
}

/// Wraps a concrete function with runtime validation of its input and
/// output, mirroring the TypeScript `validator.implement(fn)` helper.
///
/// The returned closure validates the argument against `input` (when
/// supplied) and the return value against `output` (when supplied). When a
/// validator rejects a value the closure returns an `Err` carrying the
/// validator's message, falling back to the same default messages used by
/// the TypeScript implementation. When no validators are supplied the call
/// result is returned untouched.
///
/// # Arguments
/// * `function` - The concrete function to wrap
/// * `input` - Optional validator applied to the argument before the call
/// * `output` - Optional validator applied to the return value after the call
///
/// # Returns
/// A closure that runs `function` with the configured validation
///
/// # Examples
/// ```
/// use umt_rust::validate::core::ValidateReturnType;
/// use umt_rust::validate::function::umt_func_implement;
///
/// let positive = ValidateReturnType::new("number", None, |value: &f64| *value > 0.0);
/// let wrapped = umt_func_implement(|n: f64| n * 2.0, Some(positive), None);
/// assert_eq!(wrapped(2.0), Ok(4.0));
/// assert!(wrapped(-1.0).is_err());
/// ```
pub fn umt_func_implement<I, O, F>(
    function: F,
    input: Option<ValidateReturnType<I>>,
    output: Option<ValidateReturnType<O>>,
) -> impl Fn(I) -> Result<O, String>
where
    F: Fn(I) -> O,
    I: Clone,
{
    move |argument: I| {
        if let Some(input_validator) = &input
            && !(input_validator.validate)(&argument)
        {
            return Err(input_validator
                .message
                .clone()
                .filter(|message| !message.is_empty())
                .unwrap_or_else(|| "function input at index 0 failed validation".to_string()));
        }
        let return_value = function(argument);
        if let Some(output_validator) = &output
            && !(output_validator.validate)(&return_value)
        {
            return Err(output_validator
                .message
                .clone()
                .filter(|message| !message.is_empty())
                .unwrap_or_else(|| "function output failed validation".to_string()));
        }
        Ok(return_value)
    }
}
