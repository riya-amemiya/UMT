use super::umt_average;

/// Calculates the standard deviation of a set of values.
///
/// The standard deviation is a measure of the amount of variation or dispersion
/// of a set of values. A low standard deviation indicates that the values tend
/// to be close to the mean, while a high standard deviation indicates that the
/// values are spread out over a wider range.
///
/// # Arguments
///
/// * `values` - A slice of f64 values.
///
/// # Returns
///
/// The standard deviation.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_standard_deviation;
///
/// let result = umt_standard_deviation(&[1.0, 2.0, 3.0]);
/// assert!((result - 0.816496580927726).abs() < 1e-10);
/// ```
pub fn umt_standard_deviation(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }

    let avg = umt_average(values.to_vec());

    // Calculate the squared differences from the mean
    let square_diffs: Vec<f64> = values
        .iter()
        .map(|&value| {
            let diff = value - avg;
            diff * diff
        })
        .collect();

    // Calculate the mean of the squared differences
    let avg_square_diff = umt_average(square_diffs);

    // Return the square root of the mean squared differences
    avg_square_diff.sqrt()
}
