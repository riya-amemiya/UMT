/// Calculates the nth percentile of values in an array.
///
/// # Arguments
///
/// * `array` - A slice of f64 numbers.
/// * `percentile` - Percentile value (0-100).
///
/// # Returns
///
/// The percentile value, or NaN for empty arrays.
/// Returns an error if percentile is not between 0 and 100.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_percentile;
///
/// assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 50.0).unwrap(), 3.0);
/// assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 25.0).unwrap(), 2.0);
/// assert_eq!(umt_percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 75.0).unwrap(), 4.0);
/// ```
pub fn umt_percentile(array: &[f64], percentile: f64) -> Result<f64, &'static str> {
    if array.is_empty() {
        return Ok(f64::NAN);
    }

    if !(0.0..=100.0).contains(&percentile) {
        return Err("Percentile must be between 0 and 100");
    }

    let mut sorted: Vec<f64> = array.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let index = (percentile / 100.0) * (sorted.len() - 1) as f64;
    let lower_index = index.floor() as usize;
    let upper_index = index.ceil() as usize;

    if lower_index == upper_index {
        return Ok(sorted[lower_index]);
    }

    let lower_value = sorted[lower_index];
    let upper_value = sorted[upper_index];
    let weight = index - lower_index as f64;

    Ok(lower_value + (upper_value - lower_value) * weight)
}
