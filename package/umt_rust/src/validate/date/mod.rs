//! Date validation module
//! Provides validation functionality for date values

use chrono::{DateTime, Utc};

use super::core::ValidateCoreReturnType;

/// Validates a date value
///
/// Mirrors the TypeScript `date` validator, which accepts only values that are
/// both a `Date` instance and represent a valid moment in time (rejecting
/// inputs such as `new Date("not-a-date")` whose timestamp is `NaN`). In Rust a
/// constructed `DateTime<Utc>` is always a valid moment, so the invalid-Date
/// case is modeled by passing `None`.
///
/// # Arguments
/// * `value` - The optional date to validate; `None` represents an invalid date
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::validate::date::umt_validate_date;
///
/// let valid = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
/// let result = umt_validate_date(Some(valid), None);
/// assert!(result.validate);
///
/// let invalid = umt_validate_date(None, None);
/// assert!(!invalid.validate);
/// ```
#[inline]
pub fn umt_validate_date(
    value: Option<DateTime<Utc>>,
    message: Option<&str>,
) -> ValidateCoreReturnType<Option<DateTime<Utc>>> {
    if value.is_none() {
        return ValidateCoreReturnType {
            validate: false,
            message: message.unwrap_or("").to_string(),
            type_value: value,
        };
    }

    ValidateCoreReturnType {
        validate: true,
        message: String::new(),
        type_value: value,
    }
}

/// Creates a date validator function
///
/// # Arguments
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A function that validates date values
#[allow(clippy::type_complexity)]
pub fn umt_date_validator(
    message: Option<String>,
) -> Box<dyn Fn(Option<DateTime<Utc>>) -> ValidateCoreReturnType<Option<DateTime<Utc>>>> {
    Box::new(move |value: Option<DateTime<Utc>>| {
        if value.is_none() {
            return ValidateCoreReturnType {
                validate: false,
                message: message.clone().unwrap_or_default(),
                type_value: value,
            };
        }

        ValidateCoreReturnType {
            validate: true,
            message: String::new(),
            type_value: value,
        }
    })
}
