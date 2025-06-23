/// Swaps two numbers to ensure x < y
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
    if x < y { vec![x, y] } else { vec![y, x] }
}
