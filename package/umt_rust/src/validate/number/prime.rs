//! Number validation module for prime numbers
//!
//! Provides validation functionality for checking if a number is prime.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a number is prime
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<f64>` validator for prime numbers
///
/// # Examples
///
/// ```
/// use umt_rust::validate::number::umt_prime;
///
/// let validator = umt_prime(None);
/// assert!((validator.validate)(&2.0));
/// assert!((validator.validate)(&3.0));
/// assert!((validator.validate)(&5.0));
/// assert!(!(validator.validate)(&4.0));
/// assert!(!(validator.validate)(&1.0));
/// ```
pub fn umt_prime(message: Option<&str>) -> ValidateReturnType<f64> {
    ValidateReturnType::new(
        ValueType::Number,
        |value: &f64| {
            // Check if it's a valid positive integer
            if *value <= 1.0 || value.fract() != 0.0 {
                return false;
            }

            let n = *value as i64;
            is_prime(n)
        },
        message.map(String::from),
    )
}

/// Creates a validator for checking if an integer is prime
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<i64>` validator for prime integers
pub fn umt_prime_i64(message: Option<&str>) -> ValidateReturnType<i64> {
    ValidateReturnType::new(
        ValueType::Number,
        |value: &i64| is_prime(*value),
        message.map(String::from),
    )
}

/// Internal helper function to check if a number is prime
fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as i64;
    let mut i = 5;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_true_for_prime_numbers() {
        let validator = umt_prime(None);
        assert!((validator.validate)(&2.0));
        assert!((validator.validate)(&3.0));
        assert!((validator.validate)(&5.0));
        assert!((validator.validate)(&7.0));
    }

    #[test]
    fn test_returns_false_for_non_prime_numbers() {
        let validator = umt_prime(None);
        assert!(!(validator.validate)(&4.0));
        assert!(!(validator.validate)(&6.0));
        assert!(!(validator.validate)(&8.0));
        assert!(!(validator.validate)(&9.0));
    }

    #[test]
    fn test_returns_false_for_one_and_less() {
        let validator = umt_prime(None);
        assert!(!(validator.validate)(&1.0));
        assert!(!(validator.validate)(&0.0));
        assert!(!(validator.validate)(&-2.0));
    }

    #[test]
    fn test_custom_message() {
        let custom_message = "This is not a prime number";
        let validator = umt_prime(Some(custom_message));
        assert!(!(validator.validate)(&4.0));
        assert_eq!(validator.message.as_deref(), Some(custom_message));
    }

    #[test]
    fn test_prime_i64() {
        let validator = umt_prime_i64(None);
        assert!((validator.validate)(&2));
        assert!((validator.validate)(&17));
        assert!(!(validator.validate)(&4));
        assert!(!(validator.validate)(&1));
    }

    #[test]
    fn test_decimal_numbers_are_not_prime() {
        let validator = umt_prime(None);
        assert!(!(validator.validate)(&2.5));
        assert!(!(validator.validate)(&7.1));
    }
}
