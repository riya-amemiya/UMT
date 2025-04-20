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
    let n: i32 = (x.abs().to_string().len() - 1).try_into().unwrap();
    if n > 0 {
        return vec![
            10_i32.pow(n.try_into().unwrap()),
            (x - (10_i32.pow(n.try_into().unwrap()))),
        ];
    }
    vec![0, x]
}
