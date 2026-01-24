//! Perfect square checking functionality
//!
//! This module provides a function to check if a number is a perfect square.

/// Determines if a given integer is a perfect square
///
/// A perfect square is an integer that is the square of an integer.
/// For example, 16 is a perfect square because 16 = 4^2.
///
/// # Arguments
///
/// * `n` - The number to check
///
/// # Returns
///
/// `true` if the number is a perfect square, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_perfect_square;
///
/// assert!(umt_is_perfect_square(16));
/// assert!(umt_is_perfect_square(25));
/// assert!(umt_is_perfect_square(0));
/// assert!(umt_is_perfect_square(1));
/// assert!(!umt_is_perfect_square(10));
/// assert!(!umt_is_perfect_square(-4));
/// ```
#[inline]
pub fn umt_is_perfect_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    let sqrt = (n as f64).sqrt();
    sqrt == sqrt.floor()
}

/// Determines if a given f64 value is a perfect square
///
/// For non-integer values, this returns false.
///
/// # Arguments
///
/// * `n` - The number to check
///
/// # Returns
///
/// `true` if the number is a perfect square integer, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_perfect_square_f64;
///
/// assert!(umt_is_perfect_square_f64(16.0));
/// assert!(umt_is_perfect_square_f64(4.0)); // 4.0 is treated as 4
/// assert!(!umt_is_perfect_square_f64(4.5));
/// assert!(!umt_is_perfect_square_f64(2.25)); // Non-integer input
/// ```
#[inline]
pub fn umt_is_perfect_square_f64(n: f64) -> bool {
    if n < 0.0 || !n.is_finite() {
        return false;
    }

    let sqrt = n.sqrt();
    sqrt == sqrt.floor() && sqrt.floor() * sqrt.floor() == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_squares() {
        assert!(umt_is_perfect_square(16));
        assert!(umt_is_perfect_square(25));
        assert!(umt_is_perfect_square(49));
        assert!(umt_is_perfect_square(0));
        assert!(umt_is_perfect_square(1));
    }

    #[test]
    fn test_non_perfect_squares() {
        assert!(!umt_is_perfect_square(20));
        assert!(!umt_is_perfect_square(-16));
        assert!(!umt_is_perfect_square(10));
    }

    #[test]
    fn test_large_numbers() {
        assert!(umt_is_perfect_square(100_000_000));
        assert!(!umt_is_perfect_square(100_000_002));
    }

    #[test]
    fn test_f64_perfect_squares() {
        assert!(umt_is_perfect_square_f64(16.0));
        assert!(umt_is_perfect_square_f64(4.0));
        assert!(umt_is_perfect_square_f64(0.0));
        assert!(umt_is_perfect_square_f64(1.0));
    }

    #[test]
    fn test_f64_non_perfect_squares() {
        assert!(!umt_is_perfect_square_f64(4.5));
        assert!(!umt_is_perfect_square_f64(2.25));
        assert!(!umt_is_perfect_square_f64(-4.0));
        assert!(!umt_is_perfect_square_f64(f64::NAN));
        assert!(!umt_is_perfect_square_f64(f64::INFINITY));
    }

    #[test]
    fn test_edge_cases() {
        // 0 is a perfect square (0 = 0^2)
        assert!(umt_is_perfect_square(0));
        // 1 is a perfect square (1 = 1^2)
        assert!(umt_is_perfect_square(1));
    }
}
