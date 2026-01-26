/// Calculates the median of an array of numbers.
///
/// The median is the middle value when the numbers are sorted.
/// For arrays with even length, returns the average of the two middle values.
///
/// # Arguments
///
/// * `array` - A slice of f64 numbers.
///
/// # Returns
///
/// The median value, or NaN for empty arrays.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_median;
///
/// assert_eq!(umt_median(&[1.0, 3.0, 3.0, 6.0, 7.0, 8.0, 9.0]), 6.0);
/// assert_eq!(umt_median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
/// ```
pub fn umt_median(array: &[f64]) -> f64 {
    if array.is_empty() {
        return f64::NAN;
    }

    let mut sorted: Vec<f64> = array.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mid = sorted.len() / 2;

    if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}
