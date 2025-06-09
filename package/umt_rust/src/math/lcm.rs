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
/// The least common multiple of the two input integers.
pub fn umt_lcm(a: i32, b: i32) -> i32 {
    let gcd = umt_gcd(a, b);
    let result = if gcd == 0 { 0 } else { a / gcd * b };
    result
}
