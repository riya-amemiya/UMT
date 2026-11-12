use super::{umt_get_decimal_length, umt_multiplication};

/// Performs addition without floating point errors.
///
/// This function handles floating point precision issues by scaling the numbers
/// to integers before performing addition, then scaling back.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers to add.
///
/// # Returns
///
/// The sum of all numbers.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_addition;
///
/// let result = umt_addition(&[0.1, 0.2]);
/// assert!((result - 0.3).abs() < 1e-10);
/// ```
pub fn umt_addition(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    if numbers.iter().all(|n| n.fract() == 0.0) {
        return numbers.iter().sum();
    }

    let max_decimal = numbers
        .iter()
        .map(|&n| umt_get_decimal_length(n))
        .max()
        .unwrap_or(0) as i32;
    let z = 10_f64.powi(max_decimal);

    if z.is_infinite() {
        return numbers.iter().sum();
    }

    let sum: f64 = numbers.iter().map(|&n| umt_multiplication(&[n, z])).sum();

    sum / z
}
