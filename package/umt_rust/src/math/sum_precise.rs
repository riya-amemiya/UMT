/// Computes the sum of a slice of numbers using the Neumaier
/// summation algorithm for improved floating-point precision.
///
/// # Arguments
///
/// * `numbers` - A slice of f64 numbers to sum.
///
/// # Returns
///
/// The precise sum of all numbers.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_sum_precise;
///
/// let result = umt_sum_precise(&[0.1, 0.2, 0.3]);
/// assert!((result - 0.6).abs() < 1e-15);
/// ```
pub fn umt_sum_precise(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;

    for &number in numbers {
        let t = sum + number;
        if sum.abs() >= number.abs() {
            compensation += (sum - t) + number;
        } else {
            compensation += (number - t) + sum;
        }
        sum = t;
    }

    sum + compensation
}
