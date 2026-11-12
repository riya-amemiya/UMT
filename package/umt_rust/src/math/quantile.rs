use super::{umt_multiplication, umt_percentile};

/// Calculates the q-th quantile of values using linear interpolation.
///
/// # Arguments
///
/// * `values` - A slice of f64 numbers.
/// * `q` - Quantile (0-1), equivalent to `umt_percentile(values, q * 100)`.
///
/// # Returns
///
/// The quantile value, or NaN for an empty slice.
/// Returns an error if `q` is not between 0 and 1.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_quantile;
///
/// assert_eq!(umt_quantile(&[1.0, 2.0, 3.0, 4.0, 5.0], 0.5).unwrap(), 3.0);
/// assert_eq!(umt_quantile(&[1.0, 2.0, 3.0, 4.0, 5.0], 0.25).unwrap(), 2.0);
/// assert_eq!(umt_quantile(&[1.0, 2.0, 3.0, 4.0], 0.5).unwrap(), 2.5);
/// ```
pub fn umt_quantile(values: &[f64], q: f64) -> Result<f64, &'static str> {
    umt_percentile(values, umt_multiplication(&[q, 100.0]))
}
