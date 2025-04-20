/// Separates a number into its highest power of 10 and the remaining value.
///
/// This function takes an integer as input and returns a vector containing the highest power of 10 that is less than or equal to the input number, and the remaining value after subtracting that power of 10.
///
/// # Arguments
///
/// * `x` - The integer to separate.
///
/// # Returns
///
/// A vector containing the highest power of 10 and the remaining value.
pub fn umt_math_separator(x: i32) -> Vec<i32> {
    let mut abs_x = x.abs();
    let mut power_of_10 = 1;
    while abs_x >= 10 {
        abs_x /= 10;
        power_of_10 *= 10;
    }

    if power_of_10 > 1 {
        return vec![power_of_10, x - power_of_10];
    }
    vec![0, x]
}
