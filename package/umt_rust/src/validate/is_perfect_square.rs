//! Perfect square number checking utility

/// Determines if a given integer is a perfect square
///
/// # Arguments
/// * `number` - Integer to check
///
/// # Returns
/// true if the number is a perfect square, false otherwise
///
/// # Examples
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
pub fn umt_is_perfect_square(number: i64) -> bool {
    if number < 0 {
        return false;
    }

    let sqrt = (number as f64).sqrt();
    let sqrt_floor = sqrt.floor() as i64;
    sqrt_floor * sqrt_floor == number
}

/// Determines if a given f64 integer value is a perfect square
///
/// # Arguments
/// * `number` - Number to check (must be a non-negative integer value)
///
/// # Returns
/// true if the number is a perfect square integer, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_perfect_square_f64;
///
/// assert!(umt_is_perfect_square_f64(16.0));
/// assert!(!umt_is_perfect_square_f64(2.25)); // Non-integer
/// ```
#[inline]
pub fn umt_is_perfect_square_f64(number: f64) -> bool {
    if number < 0.0 || number.fract() != 0.0 {
        return false;
    }

    let sqrt = number.sqrt();
    sqrt == sqrt.floor()
}
