/// Calculates the greatest common divisor (GCD) of two integers.
///
/// This function takes two integers as input and returns their greatest common divisor.
///
/// # Arguments
///
/// * `x` - The first integer.
/// * `y` - The second integer.
///
/// # Returns
///
/// The greatest common divisor of the two input integers.
pub fn umt_gcd(mut x: i32, mut y: i32) -> i32 {
    x = x.abs();
    y = y.abs();
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

/// Calculates the greatest common divisor (GCD) of multiple integers.
///
/// This function takes a slice of integers and returns the GCD of all of them.
/// It applies the Euclidean algorithm iteratively across all input numbers.
///
/// # Arguments
///
/// * `numbers` - A slice of integers (must contain at least 2 elements).
///
/// # Returns
///
/// The greatest common divisor of all input integers.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_gcd_multiple;
///
/// assert_eq!(umt_gcd_multiple(&[12, 18, 24]), 6);
/// assert_eq!(umt_gcd_multiple(&[56, 48, 32, 24]), 8);
/// ```
///
/// # Panics
///
/// Panics if the slice contains fewer than 2 elements.
pub fn umt_gcd_multiple(numbers: &[i32]) -> i32 {
    assert!(
        numbers.len() >= 2,
        "At least two numbers are required to compute GCD"
    );
    let mut result = umt_gcd(numbers[0], numbers[1]);
    for &num in &numbers[2..] {
        result = umt_gcd(result, num);
    }
    result
}
