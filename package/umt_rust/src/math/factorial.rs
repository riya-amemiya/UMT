/// Calculates the factorial of a number.
///
/// This function takes an integer as input and returns the factorial of that number.
///
/// # Arguments
///
/// * `x` - The integer to calculate the factorial of.
///
/// # Returns
///
/// The factorial of the input integer.
pub fn umt_factorial(mut x: i32) -> i32 {
    let mut result = 1;
    if x != 0 {
        while x > 1 {
            result *= x;
            x -= 1;
        }
    }
    return result;
}
