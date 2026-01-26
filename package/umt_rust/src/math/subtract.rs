use super::{umt_get_decimal_length, umt_max, umt_multiplication};

/// Performs subtraction with arbitrary number of arguments without floating point errors.
///
/// This function handles floating point precision issues by scaling the numbers
/// to integers before performing subtraction, then scaling back.
/// The first argument is the minuend, and all subsequent arguments are subtracted from it.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers to subtract.
///
/// # Returns
///
/// The result of the subtraction.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_subtract;
///
/// let result = umt_subtract(&[0.3, 0.1]);
/// assert!((result - 0.2).abs() < 1e-10);
///
/// let result2 = umt_subtract(&[1.0, 0.1, 0.2]);
/// assert!((result2 - 0.7).abs() < 1e-10);
/// ```
pub fn umt_subtract(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    numbers
        .iter()
        .enumerate()
        .fold(0.0, |accumulator, (index, &current)| {
            if index == 0 {
                return current;
            }
            // Get the power of 10 based on the maximum decimal places
            let z = 10_f64.powi(umt_max(&[
                umt_get_decimal_length(accumulator) as f64,
                umt_get_decimal_length(current) as f64,
            ]) as i32);
            // Scale to integers, subtract, then scale back to original decimal places
            (umt_multiplication(&[accumulator, z]) - umt_multiplication(&[current, z])) / z
        })
}
