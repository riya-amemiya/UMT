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
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}
