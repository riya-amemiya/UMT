use super::umt_gcd;

/// Calculates the least common multiple (LCM) of two integers.
///
/// This function takes two integers as input and returns their least common multiple.
///
/// # Arguments
///
/// * `a` - The first integer.
/// * `b` - The second integer.
///
/// # Returns
///
/// The least common multiple of the two input integers, which is never negative.
pub fn umt_lcm(a: i32, b: i32) -> i32 {
    let gcd = umt_gcd(a, b);

    if gcd == 0 { 0 } else { a.abs() / gcd * b.abs() }
}

/// Calculates the least common multiple (LCM) of multiple integers.
///
/// # Arguments
///
/// * `numbers` - A slice of integers (must contain at least 2 elements).
///
/// # Returns
///
/// The least common multiple of all input integers, or 0 if any of them is 0.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_lcm_multiple;
///
/// assert_eq!(umt_lcm_multiple(&[2, 3, 4]), 12);
/// assert_eq!(umt_lcm_multiple(&[4, 6, 8, 12]), 24);
/// ```
///
/// # Panics
///
/// Panics if the slice contains fewer than 2 elements.
pub fn umt_lcm_multiple(numbers: &[i32]) -> i32 {
    assert!(
        numbers.len() >= 2,
        "At least two numbers are required to compute LCM"
    );
    numbers[2..]
        .iter()
        .fold(umt_lcm(numbers[0], numbers[1]), |result, &num| {
            umt_lcm(result, num)
        })
}
