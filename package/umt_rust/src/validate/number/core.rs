//! Number validation core module
//!
//! Provides the base validation functionality for number values.

use crate::validate::types::{ValidateCoreResult, ValidateReturnType, ValueType};

/// Creates a number validator with optional validation rules
///
/// # Arguments
///
/// * `rules` - Optional array of validation rules to apply
/// * `message` - Custom error message for type validation
///
/// # Returns
///
/// A validator function that checks if the value is a number and applies validation rules
///
/// # Examples
///
/// ```
/// use umt_rust::validate::number::{umt_number_validator, umt_min_value, umt_max_value};
///
/// // Basic number validation
/// let validator = umt_number_validator(vec![], None);
/// let result = validator(5.0);
/// assert!(result.validate);
///
/// // With min/max validation rules
/// let rules = vec![umt_min_value(0.0, None), umt_max_value(10.0, None)];
/// let validator = umt_number_validator(rules, None);
/// assert!(validator(5.0).validate);
/// assert!(!validator(11.0).validate);
/// ```
pub fn umt_number_validator(
    rules: Vec<ValidateReturnType<f64>>,
    message: Option<&str>,
) -> impl Fn(f64) -> ValidateCoreResult<f64> {
    move |value: f64| {
        // Check type (in Rust, if we receive f64, it's already a number)
        // But we should check for special values
        if !value.is_finite() {
            return ValidateCoreResult::failure_optional(message, ValueType::Number);
        }

        // Apply validation rules
        for rule in &rules {
            if !(rule.validate)(&value) {
                return ValidateCoreResult {
                    validate: false,
                    message: rule.message.clone().unwrap_or_default(),
                    value_type: ValueType::Number,
                    value: Some(value),
                };
            }
        }

        ValidateCoreResult::success(value, ValueType::Number)
    }
}

/// Creates a number validator that accepts any finite number
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A validator function
pub fn umt_number_validator_simple(
    message: Option<&str>,
) -> impl Fn(f64) -> ValidateCoreResult<f64> {
    let msg = message.map(|s| s.to_string());
    move |value: f64| {
        if value.is_finite() {
            ValidateCoreResult::success(value, ValueType::Number)
        } else {
            ValidateCoreResult {
                validate: false,
                message: msg.clone().unwrap_or_default(),
                value_type: ValueType::Number,
                value: Some(value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::number::{umt_max_value, umt_min_value};

    #[test]
    fn test_number_validator_no_rules() {
        let validator = umt_number_validator(vec![], None);
        assert!(validator(5.0).validate);
        assert!(validator(-5.0).validate);
        assert!(validator(0.0).validate);
    }

    #[test]
    fn test_number_validator_with_max_value() {
        let rules = vec![umt_max_value(10.0, None)];
        let validator = umt_number_validator(rules, None);
        assert!(validator(5.0).validate);
        assert!(validator(10.0).validate);
        assert!(!validator(11.0).validate);
    }

    #[test]
    fn test_number_validator_with_min_value() {
        let rules = vec![umt_min_value(3.0, None)];
        let validator = umt_number_validator(rules, None);
        assert!(validator(5.0).validate);
        assert!(validator(3.0).validate);
        assert!(!validator(2.0).validate);
    }

    #[test]
    fn test_number_validator_with_min_and_max() {
        let rules = vec![umt_min_value(3.0, None), umt_max_value(10.0, None)];
        let validator = umt_number_validator(rules, None);
        assert!(validator(5.0).validate);
        assert!(!validator(2.0).validate);
        assert!(!validator(11.0).validate);
    }

    #[test]
    fn test_number_validator_custom_message() {
        let custom_message = "Value must be between 3 and 10";
        let rules = vec![umt_min_value(3.0, Some(custom_message))];
        let validator = umt_number_validator(rules, None);
        let result = validator(2.0);
        assert!(!result.validate);
        assert_eq!(result.message, custom_message);
    }

    #[test]
    fn test_number_validator_rejects_nan() {
        let validator = umt_number_validator(vec![], None);
        assert!(!validator(f64::NAN).validate);
    }

    #[test]
    fn test_number_validator_rejects_infinity() {
        let validator = umt_number_validator(vec![], None);
        assert!(!validator(f64::INFINITY).validate);
        assert!(!validator(f64::NEG_INFINITY).validate);
    }
}
