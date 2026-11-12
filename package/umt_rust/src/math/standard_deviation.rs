use super::umt_variance;

/// Calculates the population standard deviation of a set of values.
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
/// The standard deviation, or 0.0 for an empty slice.
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
    umt_standard_deviation_with_sample(values, false)
}

/// Calculates the standard deviation of a set of values, choosing between the
/// population and the sample variance.
///
/// # Arguments
///
/// * `values` - A slice of f64 values.
/// * `sample` - If `true`, uses the sample variance (`n - 1`) instead of the
///   population variance (`n`).
///
/// # Returns
///
/// The standard deviation, 0.0 for an empty slice, or NaN for a sample of a
/// single value.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_standard_deviation_with_sample;
///
/// assert_eq!(umt_standard_deviation_with_sample(&[1.0, 2.0, 3.0], true), 1.0);
/// let population = umt_standard_deviation_with_sample(&[1.0, 2.0, 3.0], false);
/// assert!((population - 0.816496580927726).abs() < 1e-10);
/// ```
pub fn umt_standard_deviation_with_sample(values: &[f64], sample: bool) -> f64 {
    umt_variance(values, sample).sqrt()
}
