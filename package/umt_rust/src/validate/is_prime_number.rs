//! Prime number checking functionality
//!
//! This module provides a function to check if a number is prime.

/// Determines if a number is prime
///
/// A prime number is a natural number greater than 1 that is not a product
/// of two smaller natural numbers.
///
/// # Arguments
///
/// * `n` - The number to check (must be an integer)
///
/// # Returns
///
/// `true` if the number is prime, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_prime_number;
///
/// assert!(umt_is_prime_number(2));
/// assert!(umt_is_prime_number(17));
/// assert!(!umt_is_prime_number(4));
/// assert!(!umt_is_prime_number(1));
/// assert!(!umt_is_prime_number(-3));
/// ```
#[inline]
pub fn umt_is_prime_number(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as i64;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

/// Determines if a f64 number is prime
///
/// For non-integer values, this returns false.
///
/// # Arguments
///
/// * `n` - The number to check
///
/// # Returns
///
/// `true` if the number is a prime integer, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_prime_number_f64;
///
/// assert!(umt_is_prime_number_f64(7.0));
/// assert!(!umt_is_prime_number_f64(7.5));
/// assert!(!umt_is_prime_number_f64(3.14));
/// ```
#[inline]
pub fn umt_is_prime_number_f64(n: f64) -> bool {
    // Check if it's a valid positive integer
    if n <= 1.0 || !n.is_finite() || n.fract() != 0.0 {
        return false;
    }

    umt_is_prime_number(n as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_false_for_numbers_less_than_or_equal_to_1() {
        assert!(!umt_is_prime_number(0));
        assert!(!umt_is_prime_number(1));
    }

    #[test]
    fn test_returns_true_for_prime_numbers() {
        assert!(umt_is_prime_number(2));
        assert!(umt_is_prime_number(3));
        assert!(umt_is_prime_number(5));
        assert!(umt_is_prime_number(7));
        assert!(umt_is_prime_number(11));
        assert!(umt_is_prime_number(13));
        assert!(umt_is_prime_number(17));
        assert!(umt_is_prime_number(19));
        assert!(umt_is_prime_number(23));
        assert!(umt_is_prime_number(29));
    }

    #[test]
    fn test_returns_false_for_composite_numbers() {
        assert!(!umt_is_prime_number(4));
        assert!(!umt_is_prime_number(6));
        assert!(!umt_is_prime_number(8));
        assert!(!umt_is_prime_number(9));
        assert!(!umt_is_prime_number(10));
        assert!(!umt_is_prime_number(12));
        assert!(!umt_is_prime_number(14));
        assert!(!umt_is_prime_number(15));
        assert!(!umt_is_prime_number(16));
        assert!(!umt_is_prime_number(18));
    }

    #[test]
    fn test_returns_false_for_negative_numbers() {
        assert!(!umt_is_prime_number(-2));
        assert!(!umt_is_prime_number(-7));
        assert!(!umt_is_prime_number(-11));
    }

    #[test]
    fn test_f64_non_integer_numbers() {
        assert!(!umt_is_prime_number_f64(2.5));
        assert!(!umt_is_prime_number_f64(3.14));
        assert!(umt_is_prime_number_f64(7.0)); // 7.0 is treated as integer 7
    }

    #[test]
    fn test_large_non_prime_numbers() {
        let large_non_prime = 10_000_000_000_000_i64;
        assert!(!umt_is_prime_number(large_non_prime));
    }

    #[test]
    fn test_large_prime_numbers() {
        let large_prime = 982_451_653_i64;
        assert!(umt_is_prime_number(large_prime));
    }

    #[test]
    fn test_f64_special_values() {
        assert!(!umt_is_prime_number_f64(f64::NAN));
        assert!(!umt_is_prime_number_f64(f64::INFINITY));
        assert!(!umt_is_prime_number_f64(f64::NEG_INFINITY));
    }
}
