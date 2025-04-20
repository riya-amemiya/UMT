/// Swaps the values of two numbers.
///
/// This function takes two numbers as input and returns a vector containing the swapped values.
///
/// # Arguments
///
/// * `x` - The first number.
/// * `y` - The second number.
///
/// # Returns
///
/// A vector containing the swapped values.
pub fn umt_value_swap(x: f64, y: f64) -> Vec<f64> {
    let mut x = x;
    let mut y = y;
    let temp = x;
    x = y;
    y = temp;
    vec![x, y]
}
